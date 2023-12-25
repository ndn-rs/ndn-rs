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
            KeyLocator::Name(payload) => payload.total_size(),
            KeyLocator::Digest(payload) => payload.total_size(),
        }
    }

    fn encode_value(&self, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match self {
            KeyLocator::Name(payload) => payload.encode(dst),
            KeyLocator::Digest(payload) => payload.encode(dst),
        }
    }

    fn decode_value(src: &mut BytesMut) -> Result<Self, Self::Error> {
        let r#type = VarNumber::peek(src)
            .ok_or_else(|| DecodeError::invalid("Insufficient bytes"))?
            .into();
        match r#type {
            Type::Name => Name::decode(src).map(Self::Name),
            Type::KeyDigest => KeyDigest::decode(src).map(Self::Digest),
        }
    }
}
