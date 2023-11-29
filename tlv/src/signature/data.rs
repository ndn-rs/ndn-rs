use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct DataSignature {
    info: SignatureInfo,
    value: SignatureValue,
}

impl fmt::Display for DataSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_args!("DataSignature<{} {}>", self.info, self.value).fmt(f)
    }
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

impl fmt::Display for SignatureInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "<SignatureInfo>".fmt(f)
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

impl fmt::Display for SignatureValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "<SignatureValue>".fmt(f)
    }
}
