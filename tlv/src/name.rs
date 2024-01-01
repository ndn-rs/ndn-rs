use super::*;

pub use block::FinalBlockId;
pub use component::ByteOffsetNameComponent;
pub use component::GenericNameComponent;
pub use component::ImplicitSha256DigestComponent;
pub use component::KeywordNameComponent;
pub use component::NameComponent;
pub use component::OtherTypeComponent;
pub use component::ParametersSha256DigestComponent;
pub use component::SegmentNameComponent;
pub use component::SequenceNumNameComponent;
pub use component::TimestampNameComponent;
pub use component::VersionNameComponent;

mod block;
mod component;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Tlv)]
#[tlv(r#type = Type::Name, error = DecodeError)]
pub struct Name {
    components: Vec<NameComponent>,
}

impl Name {
    pub const MAX_TLV_TYPE: Type = Type::new(65535);

    pub fn digest(digest: [u8; 32]) -> Self {
        let digest = ImplicitSha256DigestComponent::new(digest);
        let components = vec![digest.into()];
        Self { components }
    }

    pub fn from_generic(generic: Generic) -> Result<Self, DecodeError> {
        let components = generic
            .check_type(Type::Name)?
            .map(NameComponent::try_from)
            .inspect(|component| tracing::trace!(?component, "Name: decoded"))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { components })
    }
}

impl TryFrom<Generic> for Name {
    type Error = DecodeError;

    fn try_from(generic: Generic) -> Result<Self, Self::Error> {
        Self::from_generic(generic)
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

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let components = self
            .components
            .iter()
            .map(|component| component.to_string())
            .collect::<Vec<_>>()
            .join("");
        format_args!("{}", components).fmt(f)
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
