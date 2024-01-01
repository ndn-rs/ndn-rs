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
        format_args!("DataSignature<{} {}>", self.info, self.value).fmt(f)
    }
}

octets!(SignatureValue => Type::SignatureValue);

// #[derive(Clone, Debug, PartialEq, Tlv)]
// #[tlv(r#type = Type::SignatureValue, error = DecodeError)]
// pub struct SignatureValue {
//     digest: GenericArray<u8, U32>,
// }

impl SignatureValue {
    pub fn digest() -> Self {
        let digest = GenericArray::from_array([0; 32]);
        Self::new(digest)
    }
}

// impl fmt::Display for SignatureValue {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         "<SignatureValue>".fmt(f)
//     }
// }

// impl TryFrom<Generic> for SignatureValue {
//     type Error = DecodeError;

//     fn try_from(generic: Generic) -> Result<Self, Self::Error> {
//         generic
//             .check_type(Type::SignatureValue)?
//             // .self_check_length()?
//             .try_into_generic_array_inefficient()
//             .map(|digest| Self { digest })
//     }
// }

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
