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

impl fmt::Display for FinalBlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_args!("<FinalBlockId>[{}]", self.name).fmt(f)
    }
}
