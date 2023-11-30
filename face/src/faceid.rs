use super::*;

tlv::non_negative_number!(FaceId => tlv::Type::FaceId);

impl FaceId {
    pub fn null() -> Self {
        Self::from(0)
    }
}

impl fmt::Display for FaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_args!("<FaceId>[{}]", self.to_u64()).fmt(f)
    }
}
