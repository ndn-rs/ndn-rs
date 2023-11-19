use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OtherTypeComponent;

impl Tlv for OtherTypeComponent {
    fn r#type(&self) -> Type {
        todo!()
    }

    fn value(&self) -> Option<Bytes> {
        todo!()
    }

    fn payload_size(&self) -> usize {
        todo!()
    }
}
