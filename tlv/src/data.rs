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
}

impl TryFrom<Generic> for Data {
    type Error = DecodeError;

    fn try_from(generic: Generic) -> Result<Self, Self::Error> {
        let mut items = generic
            .check_type(Type::Data)?
            .self_check_length()?
            .items()
            .ok_or_else(|| DecodeError::invalid("Insufficient amount of "))?
            .into_iter();

        // Name must be first
        let name = items
            .next()
            .ok_or_else(|| DecodeError::other("Data packet must have Name as first element"))?
            .try_into()?;

        let mut metainfo: Option<MetaInfo> = None;
        let mut content: Option<Content> = None;
        let mut signature_info: Option<SignatureInfo> = None;
        let mut signature_value: Option<SignatureValue> = None;

        for item in items {
            match item.r#type() {
                Type::MetaInfo => {
                    if metainfo.is_none() {
                        metainfo = Some(MetaInfo::try_from(item)?);
                    } else {
                        Err(DecodeError::other("Multiple MetaInfio"))?
                    }
                }
                Type::Content => {
                    if content.is_none() {
                        content = Some(Content::try_from(item)?);
                    } else {
                        Err(DecodeError::other("Multiple Content"))?
                    }
                }
                Type::SignatureInfo => {
                    if content.is_none() {
                        signature_info = Some(SignatureInfo::try_from(item)?);
                    } else {
                        Err(DecodeError::other("Multiple SignatureInfo"))?
                    }
                }
                Type::SignatureValue => {
                    if content.is_none() {
                        signature_value = Some(SignatureValue::try_from(item)?);
                    } else {
                        Err(DecodeError::other("Multiple SignatureValue"))?
                    }
                }
                other => println!("skip {other}"),
            }
        }
        // let metainfo = items.next().map(MetaInfo::try_from).transpose()?;
        // let content = items.next().map(Content::try_from).transpose()?;

        // items.for_each(|item| println!("{item:?}"));

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
        write!(f, " ")?;
        display_option(&self.metainfo, f)?;
        write!(f, " ")?;
        display_option(&self.content, f)?;
        write!(f, " ")?;
        self.data_signature.fmt(f).ok();
        write!(f, "]")
    }
}
