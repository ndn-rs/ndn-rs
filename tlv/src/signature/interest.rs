use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InterestSignature {
    pub info: InterestSignatureInfo,
    pub value: InterestSignatureValue,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Tlv)]
#[tlv(r#type = Type::InterestSignatureInfo, error = DecodeError)]
pub struct InterestSignatureInfo {
    pub signature_type: SignatureType,
    pub key_locator: Option<KeyLocator>,
    pub signature_nonce: Option<SignatureNonce>,
    pub signature_time: Option<SignatureTime>,
    pub signature_seq_num: Option<SignatureSeqNum>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Tlv)]
#[tlv(r#type = Type::InterestSignatureValue, error = DecodeError)]
pub struct InterestSignatureValue;

impl fmt::Display for InterestSignatureInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "<InterestSignatureInfo>".fmt(f)
    }
}

impl fmt::Display for InterestSignatureValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "<InterestSignatureValue>".fmt(f)
    }
}

impl fmt::Display for InterestSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.info.fmt(f)?;
        self.value.fmt(f)
    }
}

impl TlvCodec for InterestSignature {
    type Error = DecodeError;
    const TYPE: Type = Type::InterestSignatureInfo;

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
