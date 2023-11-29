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

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            format_args!("<Content>[{:?}]", self.bytes()).fmt(f)
        } else {
            "<Content>[..]".fmt(f)
        }
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

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "ContentType".fmt(f)
    }
}
