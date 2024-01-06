use super::*;

pub(super) struct ClientInternal {
    read: Arc<Mutex<transport::Transport>>,
    pending: Arc<Mutex<HashMap<String, PendingInternal>>>,
}

impl ClientInternal {
    pub(super) fn new(
        read: Arc<Mutex<transport::Transport>>,
        pending: Arc<Mutex<HashMap<String, PendingInternal>>>,
    ) -> Self {
        Self { read, pending }
    }

    pub(super) async fn run(self) {
        tracing::trace!("Entering ClientInternal loop");
        loop {
            match self.next_item().await {
                Ok(generic) => {
                    tracing::trace!(?generic, "Received next_item");
                    let data = match tlv::Data::from_generic(generic) {
                        Ok(data) => data,
                        Err(err) => {
                            tracing::warn!(%err, "Dropping");
                            continue;
                        }
                    };

                    let name = data.name();
                    let pending = self
                        .pending
                        .lock()
                        .await
                        .extract_if(|key, pending| {
                            name == *key
                                || (name.starts_with(key) && pending.interest.is_can_be_prefix())
                        })
                        .map(|(name, pending)| {
                            if pending.tx.send(data.clone()).is_err() {
                                tracing::warn!(name, "Failed to send data packet; dropping");
                            }
                        })
                        .collect::<Vec<_>>();

                    if pending.is_empty() {
                        tracing::warn!(name, "Drop unsolicited data packet");
                    }
                }
                Err(err) => tracing::error!(%err, "transport.recv_item()"),
            }
        }
    }

    async fn next_item(&self) -> io::Result<tlv::Generic> {
        loop {
            let Some(item) = self.read.lock().await.recv_item().await.transpose() else {
                tracing::trace!("Empty recv_item()");
                continue;
            };
            break item;
        }
    }

    async fn _next_data_item(&self) -> io::Result<tlv::Data> {
        let generic = self.next_item().await?;
        tlv::Data::from_generic(generic)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
    }
}
