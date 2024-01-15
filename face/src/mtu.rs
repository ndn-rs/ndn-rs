use tlv::NonNegativeNumber;

use super::*;

tlv::non_negative_number!(Mtu => tlv::Type::Mtu; prefix => "mtu");

impl Mtu {
    pub const MAX_NDN_PACKET_SIZE: Self = Self(NonNegativeNumber(8800));
}
