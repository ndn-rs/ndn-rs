use std::net::SocketAddr;

use tokio::net;

use super::*;

#[derive(Debug)]
pub(super) enum Socket {
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

    pub(super) fn local(&self) -> io::Result<face::LocalUri> {
        let text = match self {
            Self::Tcp(tcp) => format!("tcp://{}", tcp.local()?),
            Self::Udp(udp) => format!("udp://{}", udp.local()?),
        };
        Ok(text.into())
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
    pub(super) async fn recv(&self) -> io::Result<Bytes> {
        todo!()
    }
}

#[derive(Debug)]
pub(super) struct Udp {
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

    fn local(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    async fn send(&self, bytes: Bytes) -> io::Result<()> {
        if self.socket.send(&bytes).await? == bytes.len() {
            Ok(())
        } else {
            Err(io::Error::other("Failed to send UDP packet"))
        }
    }
}

#[derive(Debug)]
pub(super) struct Tcp {
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

    fn local(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    async fn send(&self, bytes: Bytes) -> io::Result<()> {
        let count = loop {
            self.socket.writable().await?;

            // Try to write data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match self.socket.try_write(&bytes) {
                Ok(count) => break count,
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
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
}
