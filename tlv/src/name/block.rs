use super::*;

#[derive(Clone, Debug, PartialEq, Tlv)]
#[tlv(r#type = Type::FinalBlockId, error = DecodeError)]
pub struct FinalBlockId(pub NameComponent);

impl TryFrom<Generic> for FinalBlockId {
    type Error = DecodeError;

    fn try_from(generic: Generic) -> Result<Self, Self::Error> {
        generic
            .check_type(Type::FinalBlockId)?
            .self_check_length()?
            .items()
            .ok_or(DecodeError::other("Empty FinalBlockId"))?
            .into_iter()
            .map(NameComponent::try_from)
            .next()
            .ok_or_else(|| DecodeError::invalid("Empty FinalBlockId name"))?
            .map(FinalBlockId)
    }
}

impl fmt::Display for FinalBlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_args!("FinalBlockId=[{}]", self.0).fmt(f)
    }
}
