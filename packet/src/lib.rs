use bytes::Bytes;
use bytes::BytesMut;

use ndn_tlv as tlv;
use ndn_varnumber::VarNumber;

#[derive(Clone, Debug)]
pub struct Packet {
    r#type: tlv::Type,
    length: VarNumber,
    bytes: Bytes,
}

impl Packet {
    pub fn from_bytes(bytes: Bytes) -> Self {
        let r#type = tlv::Type::Nonce;
        let length = VarNumber::zero();
        Self {
            r#type,
            length,
            bytes,
        }
    }

    pub fn bytes(self) -> Bytes {
        self.bytes
    }

    pub fn encode(&self, dst: &mut BytesMut) {
        let r#type = self.r#type.to_varnumber().bytes();
        let length = self.length.bytes();
        let payload = self.bytes.clone();
        dst.extend([r#type, length, payload]);
    }
}

impl<T: tlv::Tlv> From<&T> for Packet {
    fn from(tlv: &T) -> Self {
        let r#type = tlv.r#type();
        let length = tlv.length();
        let bytes = tlv.bytes();
        Self {
            r#type,
            length,
            bytes,
        }
    }
}
