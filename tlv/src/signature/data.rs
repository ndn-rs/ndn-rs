use super::*;

pub use info::SignatureInfo;

mod info;

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
        format_args!("DataSignature[{} {:x}]", self.info, self.value.0).fmt(f)
    }
}

octets!(SignatureValue => Type::SignatureValue);

impl SignatureValue {
    pub fn digest() -> Self {
        let digest = GenericArray::from_array([0; 32]);
        Self::new(digest)
    }
}

impl TlvCodec for DataSignature {
    type Error = DecodeError;
    const TYPE: Type = Type::SignatureInfo;

    fn total_size(&self) -> usize {
        self.info.total_size() + self.value.total_size()
    }

    fn encode(&self, dst: &mut BytesMut) -> Result<(), Self::Error> {
        self.info.encode(dst)?;
        self.value.encode(dst)
    }

    fn decode(src: &mut BytesMut) -> Result<Self, Self::Error> {
        let info = TlvCodec::decode(src)?;
        let value = TlvCodec::decode(src)?;
        Ok(Self { info, value })
    }
}

impl TryFrom<(Option<SignatureInfo>, Option<SignatureValue>)> for DataSignature {
    type Error = DecodeError;

    fn try_from(
        data: (Option<SignatureInfo>, Option<SignatureValue>),
    ) -> Result<Self, Self::Error> {
        match data {
            (Some(info), Some(value)) => Ok(Self { info, value }),
            (Some(_info), None) => Err(DecodeError::other(
                "Invalid Data Signature (Signature Info witout Signature Value)",
            )),
            (None, Some(_value)) => Err(DecodeError::other(
                "Invalid Data Signature (Signature Value witout Signature Info)",
            )),
            (None, None) => Err(DecodeError::other(
                "Invalid Data Signature (Neither Signature Info nor Signature Value)",
            )),
        }
    }
}
