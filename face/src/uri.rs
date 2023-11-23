use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Uri(String);

impl tlv::Tlv for Uri {
    fn r#type(&self) -> tlv::Type {
        tlv::Type::Uri
    }

    fn value(&self) -> Option<Bytes> {
        let data = self.0.as_bytes();
        let bytes = Bytes::copy_from_slice(data);
        Some(bytes)
    }

    fn payload_size(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalUri(String);

impl tlv::Tlv for LocalUri {
    fn r#type(&self) -> tlv::Type {
        tlv::Type::LocalUri
    }

    fn value(&self) -> Option<Bytes> {
        let data = self.0.as_bytes();
        let bytes = Bytes::copy_from_slice(data);
        Some(bytes)
    }

    fn payload_size(&self) -> usize {
        self.0.len()
    }
}
