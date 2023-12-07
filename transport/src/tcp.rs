use super::*;

#[derive(Debug)]
pub struct Tcp {
    socket: net::TcpStream,
}

impl Tcp {
    pub(super) async fn new(
        _local: impl net::ToSocketAddrs,
        remote: impl net::ToSocketAddrs,
    ) -> io::Result<Self> {
        tracing::info!("Ignoring local for now");
        let socket = net::TcpStream::connect(remote).await?;
        Ok(Self { socket })
    }

    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    pub(super) fn face_uri(&self) -> io::Result<String> {
        let uri = format!(
            "{}{}{}",
            face::Tcp::PREFIX,
            face::URI_DELIMITER,
            self.local_addr()?,
        );
        Ok(uri)
    }

    #[tracing::instrument(level = "trace", skip_all, err(level = "error"))]
    pub(super) async fn send(&self, bytes: Bytes) -> io::Result<()> {
        let count = loop {
            self.socket.writable().await?;

            // Try to write data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match self.socket.try_write(&bytes) {
                Ok(count) => break count,
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => {
                    return Err(e);
                }
            }
        };

        if count == bytes.len() {
            Ok(())
        } else {
            Err(io::Error::other("Failed to send TCP data"))
        }
    }

    #[tracing::instrument(level = "trace", skip_all, err(level = "error"))]
    pub(super) async fn recv(&self, mut bytes: BytesMut) -> io::Result<Bytes> {
        loop {
            self.socket.readable().await?;

            let mut buf = [0; 8800];
            tracing::trace!(buffer = buf.len(), "Got buffer");

            match self.socket.try_read(&mut buf) {
                Ok(0) => break,
                Ok(count) => {
                    tracing::trace!(count, "Got bytes");
                    println!("{}", String::from_utf8_lossy(&buf[..count]));
                    bytes.extend(&buf[..count]);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => return Err(e),
            }
        }

        Ok(bytes.freeze())
    }
}
