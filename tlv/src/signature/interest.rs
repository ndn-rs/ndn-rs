use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct InterestSignature {
    info: InterestSignatureInfo,
    value: InterestSignatureValue,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InterestSignatureInfo;

#[derive(Clone, Debug, PartialEq)]
pub struct InterestSignatureValue;

impl Tlv for InterestSignatureInfo {
    fn r#type(&self) -> Type {
        Type::InterestSignatureInfo
    }

    fn value(&self) -> Option<Bytes> {
        todo!()
    }

    fn payload_size(&self) -> usize {
        todo!()
    }
}

impl Tlv for InterestSignatureValue {
    fn r#type(&self) -> Type {
        Type::InterestSignatureValue
    }

    fn value(&self) -> Option<Bytes> {
        todo!()
    }

    fn payload_size(&self) -> usize {
        todo!()
    }
}
