use tlv::NonNegativeNumber;

use super::*;

tlv::non_negative_number!(StatusCode => tlv::Type::StatusCode);
tlv::utf8_string!(StatusText => tlv::Type::StatusText);

impl StatusCode {
    pub const OK: Self = Self(NonNegativeNumber(200));
    pub const INCORRECT_CONTROL_PARAMETERS: Self = Self(NonNegativeNumber(400));
    pub const NOT_AUTHORIZED: Self = Self(NonNegativeNumber(403));
    pub const NOT_FOUND: Self = Self(NonNegativeNumber(404));
    pub const NOT_SUPPORTED: Self = Self(NonNegativeNumber(501));
    pub const SERVICE_UNAVAILABLE: Self = Self(NonNegativeNumber(503));
    pub const SOCKET_ERROR: Self = Self(NonNegativeNumber(504));

    pub fn is_ok(self) -> bool {
        self == Self::OK
    }
}
