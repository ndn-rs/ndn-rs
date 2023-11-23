use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GenericNameComponent(Vec<u8>);

impl Tlv for GenericNameComponent {
    fn r#type(&self) -> Type {
        Type::GenericNameComponent
    }

    fn value(&self) -> Option<Bytes> {
        Some(Bytes::copy_from_slice(&self.0))
    }

    fn payload_size(&self) -> usize {
        self.0.len()
    }
}
