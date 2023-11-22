use super::*;

#[derive(Debug)]
pub struct Tcp {
    pub uri: String,
    pub addr: net::SocketAddr,
}

impl FaceUri for Tcp {
    fn kick(&self) -> bool {
        false
    }

    async fn send(&self, packet: packet::Packet) -> std::io::Result<()> {
        todo!("{packet:?}")
    }

    async fn recv(&self) -> std::io::Result<packet::Packet> {
        todo!()
    }
}

impl Tcp {
    pub async fn from_uri(uri: &str) -> Result<Self, InvalidFaceUri> {
        let (tcp, addr) = split_face_uri(uri)?;
        let need_ip = need_ip(tcp).map_err(|reason| InvalidFaceUri::new(uri, reason))?;
        lookup_addr(need_ip, addr)
            .await
            .map(|addr| Self::new(uri, addr))
            .map_err(|err| InvalidFaceUri::new(uri, err))
    }

    // pub(crate) fn boxed(self) -> Box<dyn FaceUri> {
    //     Box::new(self)
    // }

    fn new(uri: &str, addr: net::SocketAddr) -> Self {
        let uri = uri.to_string();
        Self { uri, addr }
    }
}

fn need_ip(tcp: &str) -> Result<NeedIp, String> {
    match tcp {
        "tcp" => Ok(NeedIp::Any),
        "tcp4" => Ok(NeedIp::V4),
        "tcp6" => Ok(NeedIp::V6),
        _ => Err(format!("unknown tcp scheme: {tcp}")),
    }
}
