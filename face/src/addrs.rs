use std::io;

use super::*;

pub(crate) enum NeedIp {
    Any,
    V4,
    V6,
}

pub(crate) async fn lookup_addr(need_ip: NeedIp, addr: &str) -> io::Result<net::SocketAddr> {
    let mut addrs = tokio::net::lookup_host(addr).await?;

    match need_ip {
        NeedIp::Any => addrs
            .next()
            .ok_or_else(|| io::Error::other("Failed to lookup addr")),
        NeedIp::V4 => addrs
            .find(|addr| addr.is_ipv4())
            .ok_or_else(|| io::Error::other("Failed to resolve to IPv4 addr")),
        NeedIp::V6 => addrs
            .find(|addr| addr.is_ipv6())
            .ok_or_else(|| io::Error::other("Failed to resolve to IPv6 addr")),
    }
}
