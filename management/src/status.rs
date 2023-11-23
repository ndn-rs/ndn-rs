use super::*;

tlv::non_negative_number!(StatusCode => tlv::Type::StatusCode);
tlv::utf8_string!(StatusText => tlv::Type::StatusText);

impl StatusCode {
    pub fn ok() -> Self {
        200.into()
    }

    pub fn incorrect_control_parameters() -> Self {
        400.into()
    }

    pub fn not_authorized() -> Self {
        403.into()
    }

    pub fn not_found() -> Self {
        404.into()
    }

    pub fn not_supported() -> Self {
        501.into()
    }

    pub fn service_not_available() -> Self {
        503.into()
    }
}
