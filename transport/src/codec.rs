use std::io;

use bytes::Buf;
use tokio_util::codec;

use tlv::TlvCodec as _;

use super::*;

#[derive(Debug)]
pub struct TlvCodec;

impl<T: tlv::TlvCodec> codec::Encoder<T> for TlvCodec {
    type Error = io::Error;

    fn encode(&mut self, item: T, dst: &mut BytesMut) -> Result<(), Self::Error> {
        item.encode(dst)
            .map_err(|err| io::Error::other(err.to_string()))
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
        if let Some(generic) = &decoded {
            src.advance(generic.total_size());
        }

        Ok(decoded)
    }
}

#[cfg(test)]
mod tests;
