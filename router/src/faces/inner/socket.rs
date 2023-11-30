use std::net::SocketAddr;

use tokio::net;

use super::*;

use internal::Internal;
use tcp::Tcp;
use udp::Udp;

mod internal;
mod tcp;
mod udp;

#[derive(Debug)]
pub(in crate::faces) enum Socket {
    Internal(Internal),
    Tcp(Tcp),
    Udp(Udp),
}

impl Socket {
    #[tracing::instrument]
    pub(super) async fn new(local: face::Addr, remote: face::Addr) -> io::Result<Self> {
        match (local, remote) {
            (face::Addr::Internal(_), face::Addr::Internal(_)) => {
                Internal::new().await.map(Self::Internal)
            }
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
            Self::Internal(internal) => internal.face_uri(),
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
            Self::Internal(internal) => internal.send(bytes).await,
            Self::Tcp(tcp) => tcp.send(bytes).await,
            Self::Udp(udp) => udp.send(bytes).await,
        }
    }

    #[tracing::instrument]
    pub(super) async fn recv(&self, bytes: BytesMut) -> io::Result<Bytes> {
        match self {
            Self::Internal(internal) => internal.recv(bytes).await,
            Self::Tcp(tcp) => tcp.recv(bytes).await,
            Self::Udp(udp) => udp.recv(bytes).await,
        }
    }
}
