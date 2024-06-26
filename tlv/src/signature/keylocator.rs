use super::*;

pub use keydigest::KeyDigest;

mod keydigest;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    //  Tlv
)]
// #[tlv(r#type = Type::KeyLocator, error = DecodeError)]
pub enum KeyLocator {
    Name(Name),
    Digest(KeyDigest),
}

impl Tlv for KeyLocator {
    type Error = DecodeError;
    const TYPE: Type = Type::KeyLocator;

    fn length(&self) -> usize {
        match self {
            Self::Name(payload) => payload.total_size(),
            Self::Digest(payload) => payload.total_size(),
        }
    }

    fn encode_value(&self, dst: &mut BytesMut) {
        match self {
            Self::Name(payload) => payload.encode(dst),
            Self::Digest(payload) => payload.encode(dst),
        }
    }

    fn decode_value(r#type: Type, length: usize, src: &mut BytesMut) -> Result<Self, Self::Error> {
        let _ = (r#type, length);
        let r#type = VarNumber::peek(src)
            .ok_or_else(|| DecodeError::invalid("Insufficient bytes"))?
            .into();
        match r#type {
            Type::Name => Name::decode(src).map(Self::Name),
            Type::KeyDigest => KeyDigest::decode(src).map(Self::Digest),
            other => Err(DecodeError::other(format!(
                "Invalid embedded KeyLocator element {other}"
            ))),
        }
    }
}

impl fmt::Display for KeyLocator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_args!(
            "{}",
            match self {
                Self::Name(name) => name,
                Self::Digest(_) => todo!(),
            }
        )
        .fmt(f)
    }
}
