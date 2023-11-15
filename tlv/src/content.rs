use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Content {
    content: Bytes,
}

impl Tlv for Content {
    fn r#type(&self) -> Type {
        Type::Content
    }

    fn value(&self) -> Option<Bytes> {
        todo!()
    }

    fn payload_size(&self) -> usize {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContentType;

impl Tlv for ContentType {
    fn r#type(&self) -> Type {
        Type::ContentType
    }

    fn value(&self) -> Option<Bytes> {
        todo!()
    }

    fn payload_size(&self) -> usize {
        todo!()
    }
}
