use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ApplicationParameters;

impl Tlv for ApplicationParameters {
    fn r#type(&self) -> Type {
        Type::ApplicationParameters
    }

    fn value(&self) -> Option<Bytes> {
        todo!()
    }

    fn payload_size(&self) -> usize {
        todo!()
    }
}
