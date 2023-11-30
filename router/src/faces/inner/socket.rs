use std::net::SocketAddr;

use tokio::net;

use super::*;

#[derive(Debug)]
pub(in crate::faces) enum Socket {
    Tcp(Tcp),
    Udp(Udp),
}

impl Socket {
    #[tracing::instrument]
    pub(super) async fn new(local: face::Addr, remote: face::Addr) -> io::Result<Self> {
        match (local, remote) {
            (face::Addr::Tcp(local), face::Addr::Tcp(remote)) => {
                Tcp::new(local.addr, remote.addr).await.map(Self::Tcp)
            }
            (face::Addr::Udp(local), face::Addr::Udp(remote)) => {
                Udp::new(local.addr, remote.addr).await.map(Self::Udp)
            }
            _ => Err(io::Error::other("Invalid local/remote combination")),
        }
    }

    pub(super) fn local_uri(&self) -> io::Result<face::LocalUri> {
        let text = match self {
            Self::Tcp(tcp) => tcp.face_uri(),
            Self::Udp(udp) => udp.face_uri(),
        };
        text.map(Into::into)
    }

    pub(super) fn mtu(&self) -> face::Mtu {
        face::Mtu::from(1500)
    }

    #[tracing::instrument]
    pub(super) async fn send(&self, bytes: Bytes) -> io::Result<()> {
        match self {
            Self::Tcp(tcp) => tcp.send(bytes).await,
            Self::Udp(udp) => udp.send(bytes).await,
        }
    }

    #[tracing::instrument]
    pub(super) async fn recv(&self, bytes: BytesMut) -> io::Result<Bytes> {
        match self {
            Self::Tcp(tcp) => tcp.recv(bytes).await,
            Self::Udp(udp) => udp.recv(bytes).await,
        }
    }
}

#[derive(Debug)]
pub(in crate::faces) struct Tcp {
    socket: net::TcpStream,
}

impl Tcp {
    async fn new(
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

    fn face_uri(&self) -> io::Result<String> {
        let uri = format!(
            "{}{}{}",
            face::Tcp::PREFIX,
            face::URI_DELIMITER,
            self.local_addr()?,
        );
        Ok(uri)
    }

    async fn send(&self, bytes: Bytes) -> io::Result<()> {
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
    async fn recv(&self, mut bytes: BytesMut) -> io::Result<Bytes> {
        loop {
            self.socket.readable().await?;

            match self.socket.try_read(&mut bytes) {
                Ok(0) => break,
                Ok(count) => tracing::trace!(count, "Got bytes"),
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => return Err(e),
            }
        }

        Ok(bytes.freeze())
    }
}

#[derive(Debug)]
pub(in crate::faces) struct Udp {
    socket: net::UdpSocket,
}

impl Udp {
    async fn new(
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

    fn face_uri(&self) -> io::Result<String> {
        let uri = format!(
            "{}{}{}",
            face::Udp::PREFIX,
            face::URI_DELIMITER,
            self.local_addr()?,
        );
        Ok(uri)
    }

    async fn send(&self, bytes: Bytes) -> io::Result<()> {
        if self.socket.send(&bytes).await? == bytes.len() {
            Ok(())
        } else {
            Err(io::Error::other("Failed to send UDP packet"))
        }
    }

    #[tracing::instrument(level = "trace", skip_all, err(level = "error"))]
    async fn recv(&self, mut bytes: BytesMut) -> io::Result<Bytes> {
        self.socket.recv(&mut bytes).await.map(|count| {
            tracing::trace!(count, "Got bytes");
            bytes.freeze()
        })
    }
}
