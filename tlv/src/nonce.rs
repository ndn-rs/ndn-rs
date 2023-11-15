use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Nonce {
    bytes: [u8; 4],
}

impl Tlv for Nonce {
    fn r#type(&self) -> Type {
        Type::Nonce
    }

    fn value(&self) -> Option<Bytes> {
        let bytes = Bytes::copy_from_slice(&self.bytes);
        Some(bytes)
    }

    fn payload_size(&self) -> usize {
        self.bytes.len()
    }
}

impl fmt::Display for Nonce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!(
            "Nonce <{}:{}:{}:{}>",
            self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3]
        )
        .fmt(f)
    }
}
