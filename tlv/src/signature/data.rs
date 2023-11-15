use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct DataSignature {
    info: SignatureInfo,
    value: SignatureValue,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SignatureInfo {}

#[derive(Clone, Debug, PartialEq)]
pub struct SignatureValue {}

impl Tlv for SignatureInfo {
    fn r#type(&self) -> Type {
        Type::SignatureInfo
    }

    fn value(&self) -> Option<Bytes> {
        todo!()
    }

    fn payload_size(&self) -> usize {
        todo!()
    }
}

impl Tlv for SignatureValue {
    fn r#type(&self) -> Type {
        Type::SignatureValue
    }

    fn value(&self) -> Option<Bytes> {
        todo!()
    }

    fn payload_size(&self) -> usize {
        todo!()
    }
}
