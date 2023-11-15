use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MustBeFresh;

impl Tlv for MustBeFresh {
    fn r#type(&self) -> Type {
        Type::MustBeFresh
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

impl fmt::Display for MustBeFresh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "MustBeFresh".fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FreshnessPeriod {
    millis: u64,
}

impl Tlv for FreshnessPeriod {
    fn r#type(&self) -> Type {
        Type::FreshnessPeriod
    }

    fn value(&self) -> Option<Bytes> {
        todo!()
    }

    fn payload_size(&self) -> usize {
        todo!()
    }
}
