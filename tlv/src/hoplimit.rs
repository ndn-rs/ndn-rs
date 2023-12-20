use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Tlv)]
#[tlv(r#type = Type::HopLimit, error = DecodeError)]
pub struct HopLimit(pub u8);

impl HopLimit {
    pub fn new(limit: u8) -> Self {
        Self(limit)
    }
}

impl fmt::Display for HopLimit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("HopLimit<{}>", self.0).fmt(f)
    }
}
