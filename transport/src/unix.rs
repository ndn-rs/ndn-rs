use std::os::fd::AsRawFd;
use std::path::Path;

use tokio::net::unix;

use super::*;

#[derive(Debug)]
pub struct Unix {
    socket: net::UnixStream,
}

impl Unix {
    pub(super) async fn new(remote: impl AsRef<Path>) -> io::Result<Self> {
        let socket = net::UnixStream::connect(remote).await?;
        println!("Local: {:?}", socket.local_addr().unwrap());
        println!("Peer: {:?}", socket.peer_addr().unwrap());
        Ok(Self { socket })
    }

    fn local_addr(&self) -> io::Result<unix::SocketAddr> {
        self.socket.local_addr()
    }

    pub(super) fn face_uri(&self) -> io::Result<String> {
        let uri = self.unix_face_uri()?.unwrap_or_else(|| self.fd_face_uri());
        Ok(uri)
    }

    fn unix_face_uri(&self) -> io::Result<Option<String>> {
        let uri = self.local_addr()?.as_pathname().map(|path| {
            format!(
                "{}{}{}",
                face::Unix::PREFIX,
                face::URI_DELIMITER,
                path.display()
            )
        });
        Ok(uri)
    }
    fn fd_face_uri(&self) -> String {
        let fd = self.socket.as_raw_fd();
        format!("{}{}{}", "fd", face::URI_DELIMITER, fd)
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

    #[tracing::instrument(level = "trace", skip_all, ret, err(level = "error"))]
    pub(super) async fn recv(&self, mut bytes: BytesMut) -> io::Result<Bytes> {
        loop {
            tracing::trace!("Waiting for socker to become readable");
            self.socket.readable().await?;
            tracing::trace!("Socket is readable");

            let mut buf = [0; 8800];
            match self.socket.try_read(&mut buf) {
                Ok(0) => break,
                Ok(count) => {
                    tracing::trace!(count, "Got bytes");
                    bytes.extend(&buf[..count]);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => return Err(e),
            }
        }

        Ok(bytes.freeze())
    }
}
