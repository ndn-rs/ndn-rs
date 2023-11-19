use super::*;

pub use block::FinalBlockId;

mod block;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Name {
    components: Vec<NameComponent>,
    length: VarNumber,
}

impl Name {
    pub fn generic(_name: impl Into<String>) -> Self {
        todo!()
    }

    pub fn digest(digest: [u8; 32]) -> Self {
        let digest = ImplicitSha256DigestComponent::new(digest);
        let length = digest.length() + 1 + 1;
        let components = vec![NameComponent::ImplicitSha256Digest(digest)];
        Self { components, length }
    }
}

impl Tlv for Name {
    fn r#type(&self) -> Type {
        Type::Name
    }

    fn value(&self) -> Option<Bytes> {
        todo!()
    }

    fn payload_size(&self) -> usize {
        todo!()
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "Name".fmt(f)
    }
}
