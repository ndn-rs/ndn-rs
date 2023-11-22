use super::*;

#[derive(Debug)]
pub struct Udp {
    pub uri: String,
    pub addr: net::SocketAddr,
}

impl FaceUri for Udp {
    fn kick(&self) -> bool {
        false
    }

    async fn send(&self, packet: packet::Packet) -> io::Result<()> {
        todo!("{packet:?}")
    }

    async fn recv(&self) -> io::Result<packet::Packet> {
        todo!()
    }
}

impl Udp {
    pub async fn from_uri(uri: &str) -> Result<Self, InvalidFaceUri> {
        let (udp, addr) = split_face_uri(uri)?;
        let need_ip = need_ip(udp).map_err(|reason| InvalidFaceUri::new(uri, reason))?;
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

fn need_ip(udp: &str) -> Result<NeedIp, String> {
    match udp {
        "udp" => Ok(NeedIp::Any),
        "udp4" => Ok(NeedIp::V4),
        "udp6" => Ok(NeedIp::V6),
        _ => Err(format!("unknown udp scheme: {udp}")),
    }
}
