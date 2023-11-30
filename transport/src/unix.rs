use std::path::Path;

use tokio::net::unix;

use super::*;

#[derive(Debug)]
pub struct Unix {
    socket: net::UnixDatagram,
}

impl Unix {
    pub(super) async fn new(remote: impl AsRef<Path>) -> io::Result<Self> {
        let socket = net::UnixDatagram::unbound()?;
        socket.connect(remote)?;
        Ok(Self { socket })
    }

    fn local_addr(&self) -> io::Result<unix::SocketAddr> {
        self.socket.local_addr()
    }

    pub(super) fn face_uri(&self) -> io::Result<String> {
        let local = self.local_addr()?;
        let path = local
            .as_pathname()
            .ok_or_else(|| io::Error::other("Unnamed UDS"))?
            .display();
        let uri = format!("{}{}{}", face::Unix::PREFIX, face::URI_DELIMITER, path,);
        Ok(uri)
    }

    #[tracing::instrument(level = "trace", skip_all, err(level = "error"))]
    pub(super) async fn send(&self, bytes: Bytes) -> io::Result<()> {
        if self.socket.send(&bytes).await? == bytes.len() {
            Ok(())
        } else {
            Err(io::Error::other("Failed to send UDP packet"))
        }
    }

    #[tracing::instrument(level = "trace", skip_all, err(level = "error"))]
    pub(super) async fn recv(&self, mut bytes: BytesMut) -> io::Result<Bytes> {
        self.socket.recv(&mut bytes).await.map(|count| {
            tracing::trace!(count, "Got bytes");
            bytes.freeze()
        })
    }
}
