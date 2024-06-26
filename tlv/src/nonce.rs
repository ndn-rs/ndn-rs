use super::*;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    //  Tlv
)]
// #[tlv(r#type = Type::Nonce, error = DecodeError)]
pub struct Nonce {
    // octets: GenericArray<u8, U4>,
    octets: [u8; 4],
}

impl Nonce {
    pub fn generate() -> Self {
        let octets = rand::random::<[u8; 4]>()
        // .into()
        ;
        Self { octets }
    }
}

impl Tlv for Nonce {
    type Error = DecodeError;
    const TYPE: Type = Type::Nonce;

    // fn r#type(&self) -> Type {
    //     Type::Nonce
    // }

    fn length(&self) -> usize {
        self.octets.len()
    }

    fn encode_value(&self, dst: &mut BytesMut) {
        dst.put_slice(&self.octets);
    }

    fn decode_value(r#type: Type, length: usize, src: &mut BytesMut) -> Result<Self, Self::Error> {
        assert_eq!(r#type, Type::Nonce);
        let mut octets = [0; 4];
        if length == octets.len() {
            src.copy_to_slice(&mut octets);
            Ok(Self { octets })
        } else {
            Err(io::Error::other("Must have exactly four bytes").into())
        }
    }
}

impl fmt::Display for Nonce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octets = GenericArray::<u8, U4>::from_slice(&self.octets);
        if f.alternate() {
            format_args!("nonce={octets:x}").fmt(f)
        } else {
            format_args!("{}={octets:X}", self.r#type()).fmt(f)
        }
    }
}
