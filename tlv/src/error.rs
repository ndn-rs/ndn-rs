use super::*;

#[derive(Debug, Error)]
#[error("Invalid Packet data")]
pub struct DecodeError;
