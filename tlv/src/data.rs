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
    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn check_name(self, name: impl AsRef<str>) -> Result<Self, DecodeError> {
        let expected_name = name.as_ref();
        let name = self.name();

        (name == expected_name).then_some(self).ok_or_else(|| {
            DecodeError::invalid(format!(
                "Name prefix mismatch, expected: {expected_name}, found: {name}"
            ))
        })
    }

    pub fn name_starts_with(self, prefix: impl AsRef<str>) -> Result<Self, DecodeError> {
        let prefix = prefix.as_ref();
        let name = self.name();
        name.starts_with(prefix).then_some(self).ok_or_else(|| {
            DecodeError::invalid(format!(
                "Name prefix mismatch, expected: {prefix}, found: {name}"
            ))
        })
    }

    pub fn from_generic(generic: Generic) -> Result<Self, DecodeError> {
        let mut generic = generic.check_type(Type::Data)?;
        let name = generic
            .next()
            .ok_or_else(|| DecodeError::other("Data packet must have Name as first element"))?;

        let name = Name::from_generic(name)?;
        tracing::trace!(%name, "Data: decoded name");

        let mut metainfo: Option<MetaInfo> = None;
        let mut content: Option<Content> = None;
        let mut signature_info: Option<SignatureInfo> = None;
        let mut signature_value: Option<SignatureValue> = None;

        for item in generic {
            match item.r#type() {
                Type::MetaInfo => {
                    if metainfo.is_none() {
                        metainfo = MetaInfo::try_from(item)
                            .inspect(|metainfo| tracing::trace!(%metainfo, "Data: decoded"))
                            .map(Some)?;
                    } else {
                        Err(DecodeError::other("Multiple MetaInfio"))?
                    }
                }
                Type::Content => {
                    if content.is_none() {
                        content = Content::try_from(item)
                            .inspect(|content| tracing::trace!(%content, "Data: decoded"))
                            .map(Some)?;
                    } else {
                        Err(DecodeError::other("Multiple Content"))?
                    }
                }
                Type::SignatureInfo => {
                    if signature_info.is_none() {
                        signature_info = SignatureInfo::try_from(item)
                            .inspect(|info| tracing::trace!(%info, "Data: decoded"))
                            .map(Some)?;
                    } else {
                        Err(DecodeError::other("Multiple SignatureInfo"))?
                    }
                }
                Type::SignatureValue => {
                    if signature_value.is_none() {
                        signature_value = SignatureValue::try_from(item)
                            .inspect(|value| tracing::trace!(%value, "Data: decoded"))
                            .map(Some)?;
                    } else {
                        Err(DecodeError::other("Multiple SignatureValue"))?
                    }
                }
                other => tracing::warn!(%other, "Data: skip"),
            }
        }

        let data_signature = (signature_info, signature_value).try_into()?;

        Ok(Self {
            name,
            metainfo,
            content,
            data_signature,
        })
    }
}

impl TryFrom<Generic> for Data {
    type Error = DecodeError;

    fn try_from(generic: Generic) -> Result<Self, Self::Error> {
        Self::from_generic(generic)
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "<Data>[".fmt(f)?;
        self.name.fmt(f)?;
        write!(f, " ")?;
        display_option(&self.metainfo, f)?;
        write!(f, " ")?;
        display_option(&self.content, f)?;
        write!(f, " ")?;
        self.data_signature.fmt(f).ok();
        write!(f, "]")
    }
}
