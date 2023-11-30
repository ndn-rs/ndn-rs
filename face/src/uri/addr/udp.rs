use super::*;

#[derive(Debug)]
pub struct Udp {
    pub addr: net::SocketAddr,
}

impl Udp {
    pub const PREFIX: &'static str = "udp";

    pub async fn from_uri(udp: &str, addr: &str) -> io::Result<Self> {
        need_ip(udp)?.lookup_addr(addr).await.map(Self::new)
    }

    pub(super) fn any() -> Self {
        Self::new(net::SocketAddr::from((net::Ipv6Addr::UNSPECIFIED, 0)))
    }

    fn new(addr: net::SocketAddr) -> Self {
        Self { addr }
    }
}

fn need_ip(udp: &str) -> io::Result<NeedIp> {
    match udp {
        "udp" => Ok(NeedIp::Any),
        "udp4" => Ok(NeedIp::V4),
        "udp6" => Ok(NeedIp::V6),
        _ => Err(io::Error::other(format!("unknown udp scheme: {udp}"))),
    }
}
