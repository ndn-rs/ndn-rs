use bytes::{Bytes, BytesMut};

use ndn_tlv as tlv;

#[derive(Clone, Debug)]
pub struct Packet {
    bytes: Bytes,
}

impl Packet {
    pub fn bytes(self) -> Bytes {
        self.bytes
    }
}

impl<T: tlv::Tlv> From<T> for Packet {
    fn from(tlv: T) -> Self {
        let r#type = tlv.type_as_varnumber().bytes();
        let length = tlv.length().bytes();
        let payload = tlv.value().unwrap_or_default();
        let size = r#type.len() + length.len() + payload.len();
        let mut bytes = BytesMut::with_capacity(size);
        bytes.extend([r#type, length, payload]);
        let bytes = bytes.freeze();

        Self { bytes }
    }
}
