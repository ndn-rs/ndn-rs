use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Tlv)]
#[tlv(r#type = Type::MustBeFresh, error = DecodeError)]
pub struct MustBeFresh;

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
