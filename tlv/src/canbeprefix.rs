use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CanBePrefix;

impl Tlv for CanBePrefix {
    fn r#type(&self) -> Type {
        Type::CanBePrefix
    }
    fn length(&self) -> VarNumber {
        VarNumber::zero()
    }

    fn value(&self) -> Option<Bytes> {
        None
    }

    fn payload_size(&self) -> usize {
        0
    }
}

impl fmt::Display for CanBePrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "<CanBePrefix>".fmt(f)
    }
}
