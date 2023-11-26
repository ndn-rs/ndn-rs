use super::*;

pub use tcp::Tcp;
pub use udp::Udp;

mod tcp;
mod udp;

#[derive(Debug)]
pub enum Addr {
    Tcp(Tcp),
    Udp(Udp),
}

impl Addr {
    pub(super) async fn from_uri(uri: &str) -> io::Result<Self> {
        let (prefix, addr) = split_face_uri(uri)?;
        if prefix.starts_with(tcp::PREFIX) {
            Tcp::from_uri(prefix, addr).await.map(Self::Tcp)
        } else if prefix.starts_with(udp::PREFIX) {
            Udp::from_uri(prefix, addr).await.map(Self::Udp)
        } else {
            Err(io::Error::other(format!("unknown Uri prefix: {prefix}")))
        }
    }

    pub fn any(&self) -> Self {
        match self {
            Self::Tcp(_) => Self::Tcp(Tcp::any()),
            Self::Udp(_) => Self::Udp(Udp::any()),
        }
    }
}

enum NeedIp {
    Any,
    V4,
    V6,
}

impl NeedIp {
    async fn lookup_addr(&self, addr: &str) -> io::Result<net::SocketAddr> {
        let mut addrs = tokio::net::lookup_host(addr).await?;

        match self {
            Self::Any => addrs
                .next()
                .ok_or_else(|| io::Error::other("Failed to lookup addr")),
            Self::V4 => addrs
                .find(|addr| addr.is_ipv4())
                .ok_or_else(|| io::Error::other("Failed to resolve to IPv4 addr")),
            Self::V6 => addrs
                .find(|addr| addr.is_ipv6())
                .ok_or_else(|| io::Error::other("Failed to resolve to IPv6 addr")),
        }
    }
}

pub(crate) fn split_face_uri(uri: &str) -> io::Result<(&str, &str)> {
    uri.split_once(URI_DELIMITER)
        .ok_or_else(|| io::Error::other("missing '://' delimiter"))
}
