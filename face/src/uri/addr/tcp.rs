use super::*;

pub(super) const PREFIX: &str = "tcp";

#[derive(Debug)]
pub struct Tcp {
    pub addr: net::SocketAddr,
}

impl Tcp {
    pub async fn from_uri(tcp: &str, addr: &str) -> io::Result<Self> {
        need_ip(tcp)?.lookup_addr(addr).await.map(Self::new)
    }

    pub(super) fn any() -> Self {
        Self::new(net::SocketAddr::from((net::Ipv6Addr::UNSPECIFIED, 0)))
    }

    fn new(addr: net::SocketAddr) -> Self {
        Self { addr }
    }
}

fn need_ip(tcp: &str) -> io::Result<NeedIp> {
    match tcp {
        "tcp" => Ok(NeedIp::Any),
        "tcp4" => Ok(NeedIp::V4),
        "tcp6" => Ok(NeedIp::V6),
        _ => Err(io::Error::other(format!("unknown tcp scheme: {tcp}"))),
    }
}
