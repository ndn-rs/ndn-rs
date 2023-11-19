use bytes::BytesMut;

use super::*;

#[derive(Clone, Debug)]
pub struct Packet {
    bytes: Bytes,
}

impl Packet {
    pub fn bytes(self) -> Bytes {
        self.bytes
    }
}

impl<T: Tlv> From<T> for Packet {
    fn from(tlv: T) -> Self {
        let size = tlv.size();
        let mut bytes = BytesMut::with_capacity(size);
        bytes.extend_from_slice(tlv.type_as_varnumber().as_bytes());
        bytes.extend_from_slice(tlv.length().as_bytes());
        if let Some(value) = tlv.value() {
            bytes.extend(value);
        }
        let bytes = bytes.freeze();

        Self { bytes }
    }
}
