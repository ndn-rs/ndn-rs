use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Sequence {
    sequence: [u8; 8],
}

impl tlv::Tlv for Sequence {
    fn r#type(&self) -> tlv::Type {
        tlv::Type::Sequence
    }

    fn value(&self) -> Option<Bytes> {
        let bytes = Bytes::copy_from_slice(&self.sequence);
        Some(bytes)
    }

    fn payload_size(&self) -> usize {
        self.sequence.len()
    }
}
