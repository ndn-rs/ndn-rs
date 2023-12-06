use super::*;

pub use block::FinalBlockId;
pub use component::GenericNameComponent;
pub use component::ImplicitSha256DigestComponent;
pub use component::KeywordNameComponent;
pub use component::NameComponent;
pub use component::OtherTypeComponent;
pub use component::ParametersSha256DigestComponent;

mod block;
mod component;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Name {
    components: Vec<NameComponent>,
}

impl Name {
    pub const MAX_TLV_TYPE: Type = Type(65535);

    pub fn digest(digest: [u8; 32]) -> Self {
        let digest = ImplicitSha256DigestComponent::new(digest);
        let components = vec![digest.into()];
        Self { components }
    }
}

impl str::FromStr for Name {
    type Err = NameError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        text.trim()
            .trim_matches('/')
            .split('/')
            .filter(|item| !item.is_empty()) // NB! Need to recheck this
            .map(|component| component.parse())
            .collect::<Result<Vec<_>, _>>()
            .map(|components| Self { components })
    }
}

impl Tlv for Name {
    fn r#type(&self) -> Type {
        Type::Name
    }

    fn value(&self) -> Option<Bytes> {
        let items = self.components.iter().map(|component| component.bytes());
        collect_to_bytes(items)
    }

    fn payload_size(&self) -> usize {
        self.components
            .iter()
            .map(|component| component.size())
            .sum()
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let components = self
            .components
            .iter()
            .map(|component| component.to_string())
            .collect::<Vec<_>>()
            .join(",");
        format_args!("{}={}", self.r#type(), components).fmt(f)
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum NameError {
    #[error("Invalid SHA256 digest")]
    InvalidDigest,
    #[error("Invalid Component Type")]
    InvalidType,
    #[error("Component Type ({0}) out of range [1..65535]")]
    TypeOutOfRange(Type),
}
