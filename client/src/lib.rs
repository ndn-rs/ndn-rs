use std::io;
use std::marker::PhantomData;
use std::sync::Arc;

// use bytes::BytesMut;
use hashbrown::HashMap;
use tokio::sync::oneshot;
use tokio::sync::Mutex;
use tokio::task;
use tokio::time;

use ndn_face as face;
use ndn_router as router;
use ndn_tlv as tlv;
use ndn_transport as transport;

pub mod simple;
mod worker;

#[derive(Debug)]
pub struct Client {
    write: Arc<Mutex<transport::Transport>>,
    pending: Arc<Mutex<HashMap<String, PendingInternal>>>,
    worker: task::JoinHandle<()>,
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

        let inner = worker::ClientInternal::new(transport.clone(), pending.clone());
        let worker = tokio::spawn(inner.run());

        Ok(Self {
            write: transport,
            pending,
            worker,
        })
    }

    pub async fn express_interest<T>(
        &self,
        name: impl AsRef<str>,
    ) -> io::Result<PendingInterest<T>> {
        let interest = tlv::Interest::new(name).must_be_fresh().can_be_prefix();
        let (tx, rx) = oneshot::channel();
        let internal = PendingInternal {
            interest: interest.clone(),
            tx,
        };

        tracing::trace!(name = interest.name(), "About to send interest");
        let mut write = self.write.lock().await;
        tracing::trace!("Got write stream");
        write.send_item(interest.clone()).await?;
        tracing::trace!("Interest sent");

        self.pending
            .lock()
            .await
            .insert(internal.interest.name(), internal);

        Ok(PendingInterest {
            interest,
            rx,
            data: PhantomData,
        })
    }

    pub async fn shutdown(self) -> io::Result<()> {
        self.worker.await.map_err(io::Error::other)
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
    T: tlv::TlvCodec,
    <T as tlv::TlvCodec>::Error: Into<io::Error>,
{
    pub async fn data(self) -> Result<T, T::Error> {
        self.recv_item().await?.into_tlvcodec()
        // let content = data.into_content().unwrap_or_default();
        // let mut content = BytesMut::from(content.as_ref());
        // <T as tlv::TlvCodec>::decode(&mut content).map_err(Into::into)
    }

    pub fn interest(&self) -> &tlv::Interest {
        &self.interest
    }

    async fn recv_item(self) -> io::Result<tlv::Data> {
        self.rx
            .await
            .map_err(|error| io::Error::new(io::ErrorKind::BrokenPipe, error))
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
