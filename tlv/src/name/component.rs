use percent_encoding::percent_encode;
use percent_encoding::NON_ALPHANUMERIC;

use super::*;

pub use digest::ImplicitSha256DigestComponent;
pub use digest::ParametersSha256DigestComponent;
pub use generic::GenericNameComponent;
pub use keyword::KeywordNameComponent;
pub use other::OtherTypeComponent;

mod digest;
mod generic;
mod keyword;
mod other;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NameComponent {
    GenericName(GenericNameComponent),
    ImplicitSha256Digest(ImplicitSha256DigestComponent),
    ParametersSha256Digest(ParametersSha256DigestComponent),
    OtherType(OtherTypeComponent),
}

impl NameComponent {
    pub fn size(&self) -> usize {
        match self {
            Self::GenericName(c) => c.size(),
            Self::ImplicitSha256Digest(c) => c.size(),
            Self::ParametersSha256Digest(c) => c.size(),
            Self::OtherType(c) => c.size(),
        }
    }

    pub fn payload_size(&self) -> usize {
        match self {
            Self::GenericName(c) => c.payload_size(),
            Self::ImplicitSha256Digest(c) => c.payload_size(),
            Self::ParametersSha256Digest(c) => c.payload_size(),
            Self::OtherType(c) => c.payload_size(),
        }
    }

    pub fn bytes(&self) -> Bytes {
        match self {
            Self::GenericName(c) => c.bytes(),
            Self::ImplicitSha256Digest(c) => c.bytes(),
            Self::ParametersSha256Digest(c) => c.bytes(),
            Self::OtherType(c) => c.bytes(),
        }
    }

    pub fn generic(text: &str) -> Self {
        Self::GenericName(GenericNameComponent::new(text))
    }

    pub fn implicit(text: &str) -> Result<Self, NameError> {
        text.parse().map(Self::ImplicitSha256Digest)
    }

    pub fn parameters(text: &str) -> Result<Self, NameError> {
        text.parse().map(Self::ParametersSha256Digest)
    }

    pub fn other(prefix: &str, text: &str) -> Result<Self, NameError> {
        OtherTypeComponent::with_prefix(prefix, text).map(Self::OtherType)
    }
}

impl str::FromStr for NameComponent {
    type Err = NameError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if let Some((prefix, text)) = text.split_once('=') {
            match prefix {
                ImplicitSha256DigestComponent::PREFIX
                | ImplicitSha256DigestComponent::PREFIX_NUMERIC => Self::implicit(text),
                ParametersSha256DigestComponent::PREFIX
                | ParametersSha256DigestComponent::PREFIX_NUMERIC => Self::parameters(text),
                prefix => Self::other(prefix, text),
            }
        } else {
            Ok(Self::generic(text))
        }
    }
}

impl From<GenericNameComponent> for NameComponent {
    fn from(value: GenericNameComponent) -> Self {
        Self::GenericName(value)
    }
}

impl From<ImplicitSha256DigestComponent> for NameComponent {
    fn from(value: ImplicitSha256DigestComponent) -> Self {
        Self::ImplicitSha256Digest(value)
    }
}

impl From<ParametersSha256DigestComponent> for NameComponent {
    fn from(value: ParametersSha256DigestComponent) -> Self {
        Self::ParametersSha256Digest(value)
    }
}

impl From<OtherTypeComponent> for NameComponent {
    fn from(value: OtherTypeComponent) -> Self {
        Self::OtherType(value)
    }
}

impl fmt::Display for NameComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let component = match self {
            Self::GenericName(c) => c.to_string(),
            Self::ImplicitSha256Digest(c) => c.to_string(),
            Self::ParametersSha256Digest(c) => c.to_string(),
            Self::OtherType(c) => c.to_string(),
        };
        format_args!("<NameComponent>[{component}]",).fmt(f)
    }
}
