use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Tlv)]
#[tlv(r#type = Type::ForwardingHint, error = DecodeError)]
pub struct ForwardingHint;

impl fmt::Display for ForwardingHint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "<ForwardingHint>".fmt(f)
    }
}
