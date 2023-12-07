use bytes::Buf;
use tokio_util::codec;

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
    type Item = packet::Packet;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let decoded = packet::Packet::from_slice(src.as_ref());
        if let Some(packet) = &decoded {
            src.advance(packet.size());
        }
        Ok(decoded)
    }
}

#[cfg(test)]
mod tests;
