use super::*;

#[derive(Clone, Debug, PartialEq, Tlv)]
#[tlv(r#type = Type::FinalBlockId, error = DecodeError)]
pub struct FinalBlockId(pub NameComponent);

impl fmt::Display for FinalBlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_args!("FinalBlockId=[{}]", self.0).fmt(f)
    }
}
