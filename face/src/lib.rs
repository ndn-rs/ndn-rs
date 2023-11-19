use std::sync::Arc;

use bytes::Bytes;
use ndn_packet as packet;
// use ndn_tlv as tlv;

#[derive(Debug, thiserror::Error)]
#[error("NDN Fault")]
pub struct Error;

pub type NdnResult<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FaceUri {
    _uri: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Face {
    local: FaceUri,
    remote: FaceUri,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FaceRef {
    face: Arc<Face>,
}

impl Face {
    pub fn new(local: FaceUri, remote: FaceUri) -> Self {
        Self { local, remote }
    }

    pub async fn recv(&self) -> NdnResult<packet::Packet> {
        println!("Recv from local {:?}", self.local);
        let bytes = Bytes::new();
        Ok(packet::Packet::from_bytes(bytes))
    }
}
impl FaceRef {
    pub async fn send(self, data: Bytes) -> NdnResult<()> {
        // let bytes = data.bytes();
        println!(
            "Sending {} bytes packet over  {:?}",
            data.len(),
            self.face.remote
        );
        Ok(())
    }

    pub async fn recv(&self) -> NdnResult<packet::Packet> {
        self.face.recv().await
    }
}
