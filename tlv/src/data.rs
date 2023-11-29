use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Data {
    name: Name,
    metainfo: Option<MetaInfo>,
    content: Option<Content>,
    data_signature: DataSignature,
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
