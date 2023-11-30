use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GenericNameComponent(Vec<u8>);

impl GenericNameComponent {
    pub fn new(text: impl Into<Vec<u8>>) -> Self {
        Self(text.into().to_vec())
    }
}

impl Tlv for GenericNameComponent {
    fn r#type(&self) -> Type {
        Type::GenericNameComponent
    }

    fn value(&self) -> Option<Bytes> {
        Some(Bytes::copy_from_slice(&self.0))
    }

    fn payload_size(&self) -> usize {
        self.0.len()
    }
}

impl<T: Into<String>> From<T> for GenericNameComponent {
    fn from(text: T) -> Self {
        let text = text.into().into_bytes();
        Self(text)
    }
}

impl fmt::Display for GenericNameComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_args!(
            "<GenericNameComponent>[{}]",
            String::from_utf8_lossy(&self.0)
        )
        .fmt(f)
    }
}
