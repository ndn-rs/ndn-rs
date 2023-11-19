use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Fragment {
    fragment: Bytes,
}

impl tlv::Tlv for Fragment {
    fn r#type(&self) -> tlv::Type {
        tlv::Type::Fragment
    }

    fn value(&self) -> Option<Bytes> {
        Some(self.fragment.clone())
    }

    fn payload_size(&self) -> usize {
        self.fragment.len()
    }
}
