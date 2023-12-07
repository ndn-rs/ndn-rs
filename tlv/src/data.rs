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
        if generic.r#type != Type::Data {
            return Err(DecodeError::TypeMismatch(generic));
        }
        if generic.length != generic.value.len() as u64 {
            return Err(DecodeError::LengthMismatch(generic));
        }

        let Some(items) = generic.items() else {
            return Err(DecodeError::InvalidData);
        };

        // Name must be first
        let mut items = items.into_iter();
        let Some(name) = items.next() else {
            return Err(DecodeError::InvalidData);
        };

        let name = name.try_into()?;

        let metainfo = None;
        let content = None;
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
