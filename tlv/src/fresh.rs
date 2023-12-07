use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
        if f.alternate() {
            "must_be_fresh".fmt(f)
        } else {
            format_args!("{}=", self.r#type()).fmt(f)
        }
    }
}

non_negative_number!(FreshnessPeriod => Type::FreshnessPeriod);
