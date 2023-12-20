use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Tlv)]
#[tlv(r#type = Type::HopLimit, error = DecodeError)]
pub struct HopLimit(pub u8);

impl HopLimit {
    pub fn new(limit: u8) -> Self {
        Self(limit)
    }
}

// impl Tlv0 for HopLimit {
//     fn r#type(&self) -> Type {
//         Type::HopLimit
//     }

//     fn length(&self) -> VarNumber {
//         VarNumber::one()
//     }

//     fn value(&self) -> Option<Bytes> {
//         let bytes = Bytes::copy_from_slice(&[self.0]);
//         Some(bytes)
//     }

//     fn payload_size(&self) -> usize {
//         1
//     }
// }

impl fmt::Display for HopLimit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("HopLimit<{}>", self.0).fmt(f)
    }
}
