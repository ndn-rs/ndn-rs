use super::*;

#[derive(Debug)]
pub struct Client {
    face: router::Face,
}

impl Client {
    pub async fn new(remote: impl Into<face::Uri>) -> io::Result<Self> {
        let uri = remote.into();
        let face = router::Face::new(uri, None, face::FacePersistency::OnDemand, None).await?;
        Ok(Self { face })
    }

    pub async fn get<T>(&mut self, name: impl AsRef<str>) -> Result<T, T::Error>
    where
        T: tlv::TlvCodec,
    {
        let interest = tlv::Interest::new(name).must_be_fresh().can_be_prefix();
        // tracing::trace!(name = interest.name(), "About to send interest");
        self.face.send_item(interest).await?;
        // tracing::trace!("Interest sent");
        self.next_data_item().await?.into_tlvcodec::<T>()
    }

    async fn next_item(&mut self) -> io::Result<tlv::Generic> {
        // tracing::trace!("Waiting for next item");
        loop {
            let Some(item) = self.face.recv_item().await.transpose() else {
                // tracing::trace!("Empty recv_item()");
                continue;
            };
            break item;
        }
    }

    async fn next_data_item(&mut self) -> io::Result<tlv::Data> {
        let generic = self.next_item().await?;
        tlv::Data::decode_from_generic(generic)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
    }
}
