use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Tlv)]
#[tlv(r#type = Type::ImplicitSha256DigestComponent, error = DecodeError)]
pub struct ImplicitSha256DigestComponent {
    digest: GenericArray<u8, U32>,
}

impl ImplicitSha256DigestComponent {
    pub const PREFIX: &'static str = "sha256digest";
    pub const PREFIX_NUMERIC: &'static str = "1";

    pub fn new(digest: impl Into<GenericArray<u8, U32>>) -> Self {
        let digest = digest.into();
        Self { digest }
    }
}

impl TryFrom<Generic> for ImplicitSha256DigestComponent {
    type Error = DecodeError;

    fn try_from(generic: Generic) -> Result<Self, Self::Error> {
        let digest = generic
            .check_type(Type::ImplicitSha256DigestComponent)?
            .check_length(GenericArray::<u8, U32>::len())?
            .try_into_generic_array_inefficient()?;

        Ok(Self { digest })
    }
}

impl str::FromStr for ImplicitSha256DigestComponent {
    type Err = NameError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        text.parse().map_err(|_| NameError::InvalidDigest)
    }
}

impl fmt::Display for ImplicitSha256DigestComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("{}={:x}", Self::PREFIX, self.digest).fmt(f)
    }
}
