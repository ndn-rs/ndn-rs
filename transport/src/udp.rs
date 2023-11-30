use super::*;

#[derive(Debug)]
pub struct Udp {
    socket: net::UdpSocket,
}

impl Udp {
    pub(super) async fn new(
        local: impl net::ToSocketAddrs,
        remote: impl net::ToSocketAddrs,
    ) -> io::Result<Self> {
        let socket = net::UdpSocket::bind(local).await?;
        socket.connect(remote).await?;
        Ok(Self { socket })
    }

    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    pub(super) fn face_uri(&self) -> io::Result<String> {
        let uri = format!(
            "{}{}{}",
            face::Udp::PREFIX,
            face::URI_DELIMITER,
            self.local_addr()?,
        );
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
