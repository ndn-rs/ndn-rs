use tlv::NonNegativeNumber;

use super::*;

tlv::non_negative_number!(Flags => tlv::Type::Flags);
tlv::non_negative_number!(Mask => tlv::Type::Mask);

#[allow(non_upper_case_globals)]
impl Flags {
    pub const LocalFieldsEnabled: Self = Self(NonNegativeNumber(1));
    pub const LpReliabilityEnabled: Self = Self(NonNegativeNumber(2));
    pub const CongestionMarkingEnabled: Self = Self(NonNegativeNumber(4));

    pub fn empty() -> Self {
        Self(NonNegativeNumber(0))
    }

    pub fn all_fields() -> Self {
        (*Self::LocalFieldsEnabled | *Self::LpReliabilityEnabled | *Self::CongestionMarkingEnabled)
            .into()
    }
}

#[allow(non_upper_case_globals)]
impl Mask {
    pub const LocalFieldsEnabled: Self = Self(NonNegativeNumber(1));
    pub const LpReliabilityEnabled: Self = Self(NonNegativeNumber(2));
    pub const CongestionMarkingEnabled: Self = Self(NonNegativeNumber(4));

    pub fn empty() -> Self {
        Self(NonNegativeNumber(0))
    }
}
