use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ForwardingHint;

impl Tlv for ForwardingHint {
    fn r#type(&self) -> Type {
        Type::ForwardingHint
    }

    fn value(&self) -> Option<Bytes> {
        todo!()
    }

    fn payload_size(&self) -> usize {
        todo!()
    }
}
