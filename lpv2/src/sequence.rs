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

    fn decode_value(src: &mut BytesMut) -> Result<Self, Self::Error> {
        let mut sequence = [0; 8];
        if src.len() == sequence.len() {
            src.copy_to_slice(&mut sequence);
            Ok(Self { sequence })
        } else {
            Err(io::Error::other("Must have exactly four bytes").into())
        }
    }
}

// impl tlv::Tlv0 for Sequence {
//     fn r#type(&self) -> tlv::Type {
//         tlv::Type::Sequence
//     }

//     fn value(&self) -> Option<Bytes> {
//         let bytes = Bytes::copy_from_slice(&self.sequence);
//         Some(bytes)
//     }

//     fn payload_size(&self) -> usize {
//         self.sequence.len()
//     }
// }
