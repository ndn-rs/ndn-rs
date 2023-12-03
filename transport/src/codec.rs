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
    type Item = (tlv::Type, VarNumber, Bytes);
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let decoded = decode_impl(src.as_ref());
        if let Some((r#type, length, body)) = &decoded {
            let count = r#type.len() + length.len() + body.len();
            src.advance(count);
        }
        Ok(decoded)
    }
}

fn decode_impl(mut src: &[u8]) -> Option<(tlv::Type, VarNumber, Bytes)> {
    let r#type = VarNumber::from_slice(src).map(tlv::Type::from)?;
    let length = VarNumber::from_slice(src)?;
    let body_size = length.to_u64() as usize;
    let body = (src.len() > body_size).then(|| src.copy_to_bytes(body_size))?;
    Some((r#type, length, body))
}
