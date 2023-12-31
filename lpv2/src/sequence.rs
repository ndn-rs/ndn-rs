use super::*;

#[derive(Clone, Debug, PartialEq)]
// #[tlv(r#type = tlv::Type::Sequence, error = tlv::DecodeError, crates(tlv_core = tlv::core))]
pub struct Sequence {
    sequence: [u8; 8],
}

impl tlv::Tlv for Sequence {
    type Error = tlv::DecodeError;

    fn r#type(&self) -> tlv::Type {
        tlv::Type::Sequence
    }

    fn length(&self) -> usize {
        self.sequence.len()
    }

    fn encode_value(&self, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_slice(&self.sequence);
        Ok(())
    }

    fn decode_value(
        r#type: tlv::Type,
        length: usize,
        src: &mut BytesMut,
    ) -> Result<Self, Self::Error> {
        let _ = (r#type, length);
        let mut sequence = [0; 8];
        if src.len() == sequence.len() {
            src.copy_to_slice(&mut sequence);
            Ok(Self { sequence })
        } else {
            Err(io::Error::other("Must have exactly four bytes").into())
        }
    }
}
