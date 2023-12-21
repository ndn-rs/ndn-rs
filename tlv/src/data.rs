use super::*;

#[derive(Clone, Debug, Tlv)]
#[tlv(r#type = Type::Data, error = DecodeError)]
pub struct Data {
    pub name: Name,
    pub metainfo: Option<MetaInfo>,
    pub content: Option<Content>,
    pub data_signature: DataSignature,
}

impl Data {
    pub fn check_name(self, name: impl AsRef<str>) -> Result<Self, DecodeError> {
        (self.name.to_string() == name.as_ref())
            .then_some(self)
            .ok_or(DecodeError::InvalidData)
    }

    pub fn name_starts_with(self, prefix: impl AsRef<str>) -> Result<Self, DecodeError> {
        self.name
            .to_string()
            .starts_with(prefix.as_ref())
            .then_some(self)
            .ok_or(DecodeError::InvalidData)
    }
}

impl TryFrom<Generic> for Data {
    type Error = DecodeError;

    fn try_from(generic: Generic) -> Result<Self, Self::Error> {
        let mut items = generic
            .check_type(Type::Data)?
            .self_check_length()?
            .items()
            .ok_or(DecodeError::InvalidData)?
            .into_iter();

        // Name must be first
        let name = items
            .next()
            .ok_or_else(|| DecodeError::other("Data packet must have Name as first element"))?
            .try_into()?;

        let metainfo = items.next().map(MetaInfo::try_from).transpose()?;
        let content = items.next().map(Content::try_from).transpose()?;

        items.for_each(|item| println!("{item:#?}"));

        let data_signature = DataSignature::digest();

        Ok(Self {
            name,
            metainfo,
            content,
            data_signature,
        })
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "<Data>[".fmt(f)?;
        self.name.fmt(f)?;
        display_option(&self.metainfo, f)?;
        display_option(&self.content, f)?;
        self.data_signature.fmt(f).ok();
        "]".fmt(f)
    }
}
