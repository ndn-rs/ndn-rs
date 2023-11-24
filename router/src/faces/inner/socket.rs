use super::*;

#[derive(Debug)]
pub(super) struct Socket;

impl Socket {
    #[tracing::instrument]
    pub(super) async fn new(uri: &str) -> io::Result<Self> {
        Err(io::Error::other("not implemented"))
    }

    pub(super) fn local(&self) -> face::LocalUri {
        todo!()
    }

    pub(super) fn mtu(&self) -> face::Mtu {
        face::Mtu::from(1500)
    }

    #[tracing::instrument]
    pub(super) async fn send(&self, bytes: Bytes) -> io::Result<()> {
        todo!()
    }

    #[tracing::instrument]
    pub(super) async fn recv(&self) -> io::Result<Bytes> {
        todo!()
    }
}
