use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Tlv)]
#[tlv(r#type = Type::CanBePrefix, error = DecodeError)]
pub struct CanBePrefix;

impl fmt::Display for CanBePrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            "can_be_prefix".fmt(f)
        } else {
            format_args!("{}=", self.r#type()).fmt(f)
        }
    }
}
