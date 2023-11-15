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
