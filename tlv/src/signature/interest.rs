use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InterestSignature {
    info: InterestSignatureInfo,
    value: InterestSignatureValue,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InterestSignatureInfo;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

impl fmt::Display for InterestSignatureInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "<InterestSignatureInfo>".fmt(f)
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

impl fmt::Display for InterestSignatureValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "<InterestSignatureValue>".fmt(f)
    }
}
