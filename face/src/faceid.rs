use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FaceId(u64);

impl tlv::Tlv for FaceId {
    fn r#type(&self) -> tlv::Type {
        tlv::Type::FaceId
    }

    fn value(&self) -> Option<Bytes> {
        let data = self.0.to_be_bytes();
        Some(Bytes::copy_from_slice(&data))
    }

    fn payload_size(&self) -> usize {
        8
    }
}
