use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GenericNameComponent(Bytes);

impl GenericNameComponent {
    pub fn new(text: impl AsRef<[u8]>) -> Self {
        let bytes = text.as_ref();
        Self(Bytes::copy_from_slice(bytes))
    }
}

impl Tlv for GenericNameComponent {
    fn r#type(&self) -> Type {
        Type::GenericNameComponent
    }

    fn value(&self) -> Option<Bytes> {
        Some(self.0.clone())
    }

    fn payload_size(&self) -> usize {
        self.0.len()
    }
}

impl TryFrom<Generic> for GenericNameComponent {
    type Error = DecodeError;

    fn try_from(generic: Generic) -> Result<Self, Self::Error> {
        let value = generic
            .check_type(Type::GenericNameComponent)?
            .self_check_length()?
            .value;
        Ok(Self(value))
    }
}

impl<T: Into<String>> From<T> for GenericNameComponent {
    fn from(text: T) -> Self {
        let text = text.into().into();
        Self(text)
    }
}

impl fmt::Display for GenericNameComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let encoded = percent_encode(&self.0, NON_ALPHANUMERIC);
        format_args!("{encoded}").fmt(f)
    }
}
