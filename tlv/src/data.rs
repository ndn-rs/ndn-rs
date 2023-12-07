use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Data {
    pub name: Name,
    pub metainfo: Option<MetaInfo>,
    pub content: Option<Content>,
    pub data_signature: DataSignature,
}

impl Tlv for Data {
    fn r#type(&self) -> Type {
        Type::Data
    }

    fn value(&self) -> Option<Bytes> {
        todo!()
    }

    fn payload_size(&self) -> usize {
        todo!()
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

        let data_signature = DataSignature {
            info: SignatureInfo {},
            value: SignatureValue {},
        };

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
        if let Some(metainfo) = &self.metainfo {
            metainfo.fmt(f).ok();
            " ".fmt(f).ok();
        }
        if let Some(content) = &self.content {
            content.fmt(f).ok();
            " ".fmt(f).ok();
        }
        self.data_signature.fmt(f).ok();
        "]".fmt(f)
    }
}
