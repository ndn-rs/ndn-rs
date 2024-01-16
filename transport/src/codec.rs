use std::io;

use tokio_util::codec;

use super::*;

#[derive(Debug, Default)]
pub struct TlvCodec;

impl TlvCodec {
    pub fn new() -> Self {
        Self
    }
}

impl<T: tlv::TlvCodec> codec::Encoder<T> for TlvCodec {
    type Error = io::Error;

    fn encode(&mut self, item: T, dst: &mut BytesMut) -> Result<(), Self::Error> {
        item.encode(dst);
        Ok(())
    }
}

impl codec::Decoder for TlvCodec {
    type Item = tlv::Generic;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Ok(tlv::Generic::from_bytes_mut(src))
    }
}

#[cfg(test)]
mod tests;
