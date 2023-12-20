use super::*;

#[derive(Clone, Debug, PartialEq, tlv::Tlv)]
#[tlv(r#type = tlv::Type::Fragment, error = tlv::DecodeError, crates(tlv_core = tlv::core))]
pub struct Fragment {
    fragment: Bytes,
}
