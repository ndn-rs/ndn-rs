use super::*;

pub use digest::ImplicitSha256DigestComponent;
pub use digest::ParametersSha256DigestComponent;
pub use name::GenericNameComponent;
pub use other::OtherTypeComponent;

mod digest;
mod name;
mod other;

#[derive(Clone, Debug, PartialEq)]
pub enum NameComponent {
    GenericName(GenericNameComponent),
    ImplicitSha256Digest(ImplicitSha256DigestComponent),
    ParametersSha256Digest(ParametersSha256DigestComponent),
    OtherType(OtherTypeComponent),
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
