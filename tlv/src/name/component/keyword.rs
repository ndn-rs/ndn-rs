use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeywordNameComponent(Vec<u8>);

impl KeywordNameComponent {
    pub fn new(text: impl AsRef<[u8]>) -> Self {
        Self(text.as_ref().to_vec())
    }
}

impl Tlv for KeywordNameComponent {
    fn r#type(&self) -> Type {
        Type::KeywordNameComponent
    }

    fn value(&self) -> Option<Bytes> {
        Some(Bytes::copy_from_slice(&self.0))
    }

    fn payload_size(&self) -> usize {
        self.0.len()
    }
}

impl<T: Into<String>> From<T> for KeywordNameComponent {
    fn from(text: T) -> Self {
        let text = text.into().into_bytes();
        Self(text)
    }
}

impl fmt::Display for KeywordNameComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let encoded = percent_encode(&self.0, NON_ALPHANUMERIC);
        format_args!("{encoded}").fmt(f)
    }
}
