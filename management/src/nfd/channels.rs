use core::fmt;

use super::*;

#[derive(Clone, Debug, tlv::Tlv)]
#[tlv(r#type = tlv::Type::ChannelStatus, error = tlv::DecodeError, crates(tlv_core = "tlv::core"))]
pub struct ChannelStatus {
    pub local_uri: face::LocalUri,
}

impl ChannelStatus {
    pub const NAME: &'static str = "/localhost/nfd/faces/channels";
}

impl fmt::Display for ChannelStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.local_uri.fmt(f)
    }
}
