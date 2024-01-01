use bytes::Buf;
use bytes::Bytes;
use bytes::BytesMut;

use ndn_tlv as tlv;
use ndn_varnumber::VarNumber;

#[derive(Clone, Debug)]
pub struct Packet {
    pub r#type: tlv::Type,
    pub length: VarNumber,
    pub value: Bytes,
}

impl Packet {
    pub fn from_slice(mut src: &[u8]) -> Option<Self> {
        let r#type = tlv::Type::from_buf(&mut src)?;
        let length = VarNumber::from_buf(&mut src)?;
        let value_size = length.to_usize();
        let value = (src.len() >= value_size).then(|| src.copy_to_bytes(value_size))?;
        Some(Self {
            r#type,
            length,
            value,
        })
    }

    pub fn value(self) -> Bytes {
        self.value
    }

    pub fn size(&self) -> usize {
        self.r#type.to_varnumber().len() + self.length.len() + self.value.len()
    }

    pub fn encode(&self, dst: &mut BytesMut) {
        let r#type = self.r#type.to_varnumber().bytes();
        let length = self.length.bytes();
        let value = self.value.clone();
        dst.extend([r#type, length, value]);
    }

    pub fn to_data(&self) -> Option<tlv::Data> {
        None
    }
}
