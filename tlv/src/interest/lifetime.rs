use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InterestLifetime;

impl Tlv for InterestLifetime {
    fn r#type(&self) -> Type {
        Type::InterestLifetime
    }

    fn value(&self) -> Option<Bytes> {
        todo!()
    }

    fn payload_size(&self) -> usize {
        todo!()
    }
}
