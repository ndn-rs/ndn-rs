use super::*;

#[derive(Clone, Debug)]
pub struct DataSignature {
    pub info: SignatureInfo,
    pub value: SignatureValue,
}

impl DataSignature {
    pub fn digest() -> Self {
        let info = SignatureInfo::digest();
        let value = SignatureValue::digest();

        Self { info, value }
    }
}

impl fmt::Display for DataSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_args!("DataSignature<{} {}>", self.info, self.value).fmt(f)
    }
}

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

#[derive(Clone, Debug, PartialEq, Tlv)]
#[tlv(r#type = Type::SignatureValue, error = DecodeError)]
pub struct SignatureValue {
    digest: GenericArray<u8, U32>,
}

impl SignatureValue {
    pub fn digest() -> Self {
        let digest = GenericArray::from_array([0; 32]);
        Self { digest }
    }
}

impl fmt::Display for SignatureInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "<SignatureInfo>".fmt(f)
    }
}

impl fmt::Display for SignatureValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "<SignatureValue>".fmt(f)
    }
}

impl TlvCodec for DataSignature {
    type Error = DecodeError;

    fn total_size(&self) -> usize {
        self.info.total_size() + self.value.total_size()
    }

    fn encode(&self, dst: &mut BytesMut) -> Result<(), Self::Error> {
        self.info.encode(dst)?;
        self.value.encode(dst)
    }

    fn decode(src: &mut BytesMut) -> Result<Self, Self::Error> {
        let _ = src;
        todo!("Need to think how to decode both info and value at once")
    }
}
