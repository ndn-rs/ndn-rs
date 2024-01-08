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

    pub fn metainfo(&self) -> Option<&MetaInfo> {
        self.metainfo.as_ref()
    }

    pub fn content_type(&self) -> Option<ContentType> {
        self.metainfo.as_ref()?.content_type
    }

    pub fn signature(&self) -> &DataSignature {
        &self.data_signature
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

    pub fn decode_from_generic(generic: Generic) -> Result<Self, DecodeError> {
        let Generic {
            r#type,
            length,
            mut value,
        } = generic.check_type(Type::Data)?;
        let length = length.to_usize();
        Self::decode_value(r#type, length, &mut value)
    }

    pub fn into_content(self) -> Option<Bytes> {
        self.content.map(|content| content.0)
    }

    pub fn into_tlvcodec<T>(self) -> Result<T, T::Error>
    where
        T: TlvCodec,
    {
        let content = self.into_content().unwrap_or_default();
        let mut content = BytesMut::from(content.as_ref());
        <T as TlvCodec>::decode(&mut content)
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
