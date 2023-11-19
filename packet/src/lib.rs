use bytes::Bytes;

use ndn_tlv as tlv;

#[derive(Clone, Debug)]
pub struct Packet {
    bytes: Bytes,
}

impl Packet {
    pub fn from_bytes(bytes: Bytes) -> Self {
        Self { bytes }
    }

    pub fn bytes(self) -> Bytes {
        self.bytes
    }
}

impl<T: tlv::Tlv> From<&T> for Packet {
    fn from(tlv: &T) -> Self {
        let bytes = tlv.bytes();
        Self { bytes }
    }
}
