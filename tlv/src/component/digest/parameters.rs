use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParametersSha256DigestComponent {
    digest: GenericArray<u8, U32>,
}

impl ParametersSha256DigestComponent {
    pub fn new(digest: impl Into<GenericArray<u8, U32>>) -> Self {
        let digest = digest.into();
        Self { digest }
    }
}

impl Tlv for ParametersSha256DigestComponent {
    fn r#type(&self) -> Type {
        Type::ParametersSha256DigestComponent
    }

    fn value(&self) -> Option<Bytes> {
        todo!()
    }

    fn payload_size(&self) -> usize {
        todo!()
    }
}

impl fmt::Display for ParametersSha256DigestComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("params-sha256={:x}", self.digest).fmt(f)
    }
}
