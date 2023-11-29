use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GenericNameComponent(Vec<u8>);

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

impl fmt::Display for GenericNameComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_args!(
            "<GenericNameComponent>[{}]",
            String::from_utf8_lossy(&self.0)
        )
        .fmt(f)
    }
}
