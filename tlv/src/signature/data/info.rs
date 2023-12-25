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

impl TryFrom<Generic> for SignatureInfo {
    type Error = DecodeError;

    fn try_from(generic: Generic) -> Result<Self, Self::Error> {
        let mut items = generic
            .check_type(Type::SignatureInfo)?
            .self_check_length()?
            .items()
            .ok_or_else(|| DecodeError::other("Empty SignatureInfo"))?
            .into_iter();

        let signature_type = items
            .next()
            .ok_or_else(|| {
                DecodeError::other("SignatureInfo must have SignatureType as first element")
            })?
            .try_into()?;

        let key_locator = if signature_type.needs_key_locator() {
            items
                .next()
                .ok_or_else(|| {
                    DecodeError::other("SignatureType requires KeyLocator, which is missing")
                })?
                .try_into::<KeyLocator>()?
        } else {
            None
        };

        Ok(Self {
            signature_type,
            key_locator,
        })
    }
}
