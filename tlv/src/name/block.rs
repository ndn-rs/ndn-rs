use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct FinalBlockId {
    name: NameComponent,
}

impl Tlv for FinalBlockId {
    fn r#type(&self) -> Type {
        Type::FinalBlockId
    }

    fn value(&self) -> Option<Bytes> {
        todo!()
    }

    fn payload_size(&self) -> usize {
        todo!()
    }
}
