use super::*;

pub use keydigest::KeyDigest;

mod keydigest;

#[derive(
    Clone,
    Debug,
    //  Tlv
)]
// #[tlv(r#type = Type::KeyLocator, error = DecodeError)]
pub enum KeyLocator {
    Name(Name),
    Digest(KeyDigest),
}

impl Tlv for KeyLocator {
    type Error = DecodeError;

    fn r#type(&self) -> Type {
        Type::KeyLocator
    }

    fn length(&self) -> usize {
        match self {
            Self::Name(payload) => payload.total_size(),
            Self::Digest(payload) => payload.total_size(),
        }
    }

    fn encode_value(&self, dst: &mut BytesMut) -> Result<(), Self::Error> {
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

impl TryFrom<Generic> for KeyLocator {
    type Error = DecodeError;

    fn try_from(generic: Generic) -> Result<Self, Self::Error> {
        let length = generic.length();
        let bytes = generic
            .check_type(Type::KeyLocator)?
            // .self_check_length()?
            .value;
        let mut src = BytesMut::from(bytes.as_ref());
        Self::decode_value(Type::KeyLocator, length, &mut src)
    }
}
