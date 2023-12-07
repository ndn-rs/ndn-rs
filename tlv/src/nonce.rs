use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Nonce {
    octets: GenericArray<u8, U4>,
}

impl Nonce {
    pub fn generate() -> Self {
        let octets = rand::random::<[u8; 4]>().into();
        Self { octets }
    }
}

impl Tlv for Nonce {
    fn r#type(&self) -> Type {
        Type::Nonce
    }

    fn value(&self) -> Option<Bytes> {
        let bytes = Bytes::copy_from_slice(&self.octets);
        Some(bytes)
    }

    fn payload_size(&self) -> usize {
        self.octets.len()
    }
}

impl fmt::Display for Nonce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            format_args!("nonce={:x}", self.octets).fmt(f)
        } else {
            format_args!("{}={:x}", self.r#type(), self.octets).fmt(f)
        }
    }
}
