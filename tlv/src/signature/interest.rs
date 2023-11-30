use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InterestSignature {
    pub info: InterestSignatureInfo,
    pub value: InterestSignatureValue,
}

impl Tlv for InterestSignature {
    fn r#type(&self) -> Type {
        panic!("This object doesn't have its own TLV-TYPE")
    }

    fn value(&self) -> Option<Bytes> {
        let items = [self.info.bytes(), self.value.bytes()];
        collect_to_bytes(items)
    }

    fn size(&self) -> usize {
        self.info.size() + self.value.size()
    }

    fn payload_size(&self) -> usize {
        panic!("This object doesn't have its own payload")
    }
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
