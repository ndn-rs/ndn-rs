use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitSha256DigestComponent {
    digest: GenericArray<u8, U32>,
}

impl ImplicitSha256DigestComponent {
    pub fn new(digest: impl Into<GenericArray<u8, U32>>) -> Self {
        let digest = digest.into();
        Self { digest }
    }
}

impl Tlv for ImplicitSha256DigestComponent {
    fn r#type(&self) -> Type {
        Type::ImplicitSha256DigestComponent
    }

    fn value(&self) -> Option<Bytes> {
        let bytes = Bytes::copy_from_slice(&self.digest);
        Some(bytes)
    }

    fn payload_size(&self) -> usize {
        32
    }
}

impl fmt::Display for ImplicitSha256DigestComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("sha256digest={:x}", self.digest).fmt(f)
    }
}
