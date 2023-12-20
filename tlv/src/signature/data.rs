use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct DataSignature {
    pub info: SignatureInfo,
    pub value: SignatureValue,
}

impl fmt::Display for DataSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_args!("DataSignature<{} {}>", self.info, self.value).fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Tlv)]
#[tlv(r#type = Type::SignatureInfo, error = DecodeError)]
pub struct SignatureInfo;

#[derive(Clone, Debug, PartialEq, Tlv)]
#[tlv(r#type = Type::SignatureValue, error = DecodeError)]
pub struct SignatureValue;

// impl Tlv0 for SignatureInfo {
//     fn r#type(&self) -> Type {
//         Type::SignatureInfo
//     }

//     fn value(&self) -> Option<Bytes> {
//         todo!()
//     }

//     fn payload_size(&self) -> usize {
//         todo!()
//     }
// }

impl fmt::Display for SignatureInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "<SignatureInfo>".fmt(f)
    }
}

// impl Tlv0 for SignatureValue {
//     fn r#type(&self) -> Type {
//         Type::SignatureValue
//     }

//     fn value(&self) -> Option<Bytes> {
//         todo!()
//     }

//     fn payload_size(&self) -> usize {
//         todo!()
//     }
// }

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
