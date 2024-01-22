use super::*;

// FaceScope indicates whether the face is local for scope control purposes.
tlv::non_negative_number!(FaceScope => tlv::Type::FaceScope; display_as_str);

#[allow(non_upper_case_globals)]
impl FaceScope {
    pub const NonLocal: Self = Self(tlv::NonNegativeNumber(0));
    pub const Local: Self = Self(tlv::NonNegativeNumber(1));

    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::NonLocal => "non-local",
            Self::Local => "local",
            _ => "unknown",
        }
    }
}
