use super::*;

#[derive(Clone, Debug, Tlv)]
#[tlv(r#type = Type::SignatureInfo, error = DecodeError)]
pub struct SignatureInfo {
    pub signature_type: SignatureType,
    pub key_locator: Option<KeyLocator>,
}

impl SignatureInfo {
    pub fn digest() -> Self {
        Self {
            signature_type: SignatureType::DigestSha256,
            key_locator: None,
        }
    }
}

impl fmt::Display for SignatureInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.signature_type.fmt(f)?;
        if let Some(key_locator) = &self.key_locator {
            format_args!(" ({key_locator})").fmt(f)
        } else {
            Ok(())
        }
    }
}
