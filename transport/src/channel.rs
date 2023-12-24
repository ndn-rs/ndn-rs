use super::*;

#[derive(Debug)]
pub enum Channel {
    Tcp(net::TcpListener),
}

impl Channel {
    pub async fn new(local: face::Addr) -> io::Result<Self> {
        match local {
            face::Addr::Internal(_) => todo!(),
            face::Addr::Tcp(local) => Self::tcp(local.addr).await,
            face::Addr::Udp(_) => todo!(),
            face::Addr::Unix(_) => todo!(),
        }
    }

    async fn tcp(addr: impl net::ToSocketAddrs) -> io::Result<Self> {
        net::TcpListener::bind(addr).await.map(Self::Tcp)
    }
}
