use std::collections::HashMap;
use std::io;
use std::marker::PhantomData;
use std::sync::Arc;

use bytes::BytesMut;
use tokio::sync::oneshot;
use tokio::sync::Mutex;

use ndn_face as face;
use ndn_tlv as tlv;
use ndn_transport as transport;

mod worker;

#[derive(Clone, Debug)]
pub struct Client {
    transport: Arc<Mutex<transport::Transport>>,
    pending: Arc<Mutex<HashMap<String, PendingInternal>>>,
}

impl Client {
    pub async fn new(remote: impl Into<face::Uri>) -> io::Result<Self> {
        let remote = remote.into().to_addr().await?;
        let local = remote.any();
        let pending = Arc::new(Mutex::new(HashMap::new()));
        let transport = transport::Transport::new(local, remote)
            .await
            .map(Mutex::new)
            .map(Arc::new)?;

        let client = Self { transport, pending };
        let _handle = tokio::spawn(client.clone().run());

        Ok(client)
    }

    pub async fn express_interest<T>(&self, name: impl AsRef<str>) -> PendingInterest<T> {
        let interest = tlv::Interest::new(name);
        let (tx, rx) = oneshot::channel();
        let internal = PendingInternal {
            interest: interest.clone(),
            tx,
        };

        self.pending
            .lock()
            .await
            .insert(internal.interest.name(), internal);

        PendingInterest {
            interest,
            rx,
            data: PhantomData,
        }
    }
    async fn run(self) {
        loop {
            match self.next_item().await {
                Ok(generic) => {
                    let data =
                        tlv::Data::from_generic(generic).expect("Should be valid data packet");
                    let name = data.name();
                    let Some(pending) = self.pending.lock().await.remove(&name) else {
                        tracing::warn!(name, "Drop unsolicited data packet");
                        continue;
                    };
                    if pending.tx.send(data).is_err() {
                        tracing::warn!(name, "Failed to send data packet; dropping");
                    }
                }
                Err(err) => tracing::error!(%err, "transport.recv_item()"),
            }
        }
    }

    async fn next_item(&self) -> io::Result<tlv::Generic> {
        loop {
            let Some(item) = self.transport.lock().await.recv_item().await.transpose() else {
                continue;
            };
            break item;
        }
    }
}

#[derive(Debug)]
struct PendingInternal {
    interest: tlv::Interest,
    tx: oneshot::Sender<tlv::Data>,
}

#[derive(Debug)]
pub struct PendingInterest<T> {
    interest: tlv::Interest,
    rx: oneshot::Receiver<tlv::Data>,
    data: PhantomData<T>,
}

impl<T> PendingInterest<T>
where
    T: tlv::Tlv,
    <T as tlv::Tlv>::Error: 'static,
{
    pub async fn data(self) -> io::Result<T> {
        let content = self
            .rx
            .await
            .map_err(io::Error::other)?
            .into_content()
            .unwrap_or_default();
        let mut content = BytesMut::from(content.as_ref());
        <T as tlv::TlvCodec>::decode(&mut content).map_err(io::Error::other)
    }

    pub fn interest(&self) -> &tlv::Interest {
        &self.interest
    }
}

// impl<T> PendingInterest<Option<T>>
// where
//     T: tlv::Tlv,
// {
//     pub async fn data(self) -> io::Result<T> {
//         let content = self.rx.await.map_err(io::Error::other)?.into_content();
//         if let Some(content) = content {
//             let mut content = BytesMut::from(content.as_ref());
//             <T as tlv::TlvCodec>::decode(&mut content).map_err(io::Error::other)
//         } else {
//             Ok(None)
//         }
//     }
// }
