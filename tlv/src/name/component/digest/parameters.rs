use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Tlv)]
#[tlv(r#type = Type::ParametersSha256DigestComponent, error = DecodeError)]
pub struct ParametersSha256DigestComponent {
    digest: GenericArray<u8, U32>,
}

impl ParametersSha256DigestComponent {
    pub const PREFIX: &'static str = "params-sha256";
    pub const PREFIX_NUMERIC: &'static str = "2";

    pub fn new(digest: impl Into<GenericArray<u8, U32>>) -> Self {
        let digest = digest.into();
        Self { digest }
    }
}

impl str::FromStr for ParametersSha256DigestComponent {
    type Err = NameError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        text.parse().map_err(|_| NameError::InvalidDigest)
    }
}

impl fmt::Display for ParametersSha256DigestComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("{}={:x}", Self::PREFIX, self.digest).fmt(f)
    }
}
