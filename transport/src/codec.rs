use std::io;

use bytes::Buf;
use tokio_util::codec;

use tlv::Tlv as _;

use super::*;

#[derive(Debug)]
pub struct TlvCodec;

impl<T: tlv::Tlv> codec::Encoder<T> for TlvCodec {
    type Error = io::Error;

    fn encode(&mut self, item: T, dst: &mut BytesMut) -> Result<(), Self::Error> {
        item.write(dst);
        Ok(())
    }
}

impl codec::Decoder for TlvCodec {
    type Item = tlv::Generic;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let decoded = {
            let mut src = io::Cursor::new(src.as_ref());
            tlv::Generic::from_buf(&mut src)
        };
        if let Some(packet) = &decoded {
            src.advance(packet.size());
        }

        Ok(decoded)
    }
}

#[cfg(test)]
mod tests;
