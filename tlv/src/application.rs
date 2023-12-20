use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Tlv)]
#[tlv(r#type = Type::ApplicationParameters, error = DecodeError)]
pub struct ApplicationParameters;

impl fmt::Display for ApplicationParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "<ApplicationParameters>".fmt(f)
    }
}
