use super::*;

pub use keydigest::KeyDigest;

mod keydigest;

#[derive(Clone, Debug, Tlv)]
#[tlv(r#type = Type::KeyLocator, error = DecodeError)]
pub struct KeyLocator;
