use super::*;

tlv::non_negative_number!(FaceId => tlv::Type::FaceId; prefix => "faceid");

impl FaceId {
    pub fn null() -> Self {
        Self::from(0)
    }
}
