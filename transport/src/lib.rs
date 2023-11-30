use std::io;
use std::net::SocketAddr;

use bytes::Bytes;
use bytes::BytesMut;
use tokio::net;

use ndn_face as face;
// use ndn_tlv as tlv;

use internal::Internal;
use tcp::Tcp;
use udp::Udp;
use unix::Unix;

mod internal;
mod tcp;
mod udp;
mod unix;

#[derive(Debug)]
pub enum Transport {
    Internal(Internal),
    Tcp(Tcp),
    Udp(Udp),
    Unix(Unix),
}

impl Transport {
    #[tracing::instrument]
    pub async fn new(local: face::Addr, remote: face::Addr) -> io::Result<Self> {
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
            (face::Addr::Unix(_local), face::Addr::Unix(remote)) => {
                Unix::new(remote.path).await.map(Self::Unix)
            }
            _ => Err(io::Error::other("Invalid local/remote combination")),
        }
    }

    pub fn local_uri(&self) -> io::Result<face::LocalUri> {
        let text = match self {
            Self::Internal(internal) => internal.face_uri(),
            Self::Tcp(tcp) => tcp.face_uri(),
            Self::Udp(udp) => udp.face_uri(),
            Self::Unix(unix) => unix.face_uri(),
        };
        text.map(Into::into)
    }

    pub fn mtu(&self) -> face::Mtu {
        face::Mtu::from(1500)
    }

    #[tracing::instrument]
    pub async fn send(&self, bytes: Bytes) -> io::Result<()> {
        match self {
            Self::Internal(internal) => internal.send(bytes).await,
            Self::Tcp(tcp) => tcp.send(bytes).await,
            Self::Udp(udp) => udp.send(bytes).await,
            Self::Unix(unix) => unix.send(bytes).await,
        }
    }

    #[tracing::instrument]
    pub async fn recv(&self, bytes: BytesMut) -> io::Result<Bytes> {
        match self {
            Self::Internal(internal) => internal.recv(bytes).await,
            Self::Tcp(tcp) => tcp.recv(bytes).await,
            Self::Udp(udp) => udp.recv(bytes).await,
            Self::Unix(unix) => unix.recv(bytes).await,
        }
    }
}
