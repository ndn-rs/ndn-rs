use super::*;

tlv::non_negative_number!(StatusCode => tlv::Type::StatusCode);
tlv::utf8_string!(StatusText => tlv::Type::StatusText);
