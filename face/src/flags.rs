use tlv::NonNegativeNumber;

use super::*;

tlv::non_negative_number!(Flags => tlv::Type::Flags; skip_display);
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

    pub fn local_fields_enabled(self) -> bool {
        *self & *Self::LocalFieldsEnabled != 0
    }

    pub fn lp_reliability_enabled(self) -> bool {
        *self & *Self::LpReliabilityEnabled != 0
    }

    pub fn congestion_marking_enabled(self) -> bool {
        *self & *Self::CongestionMarkingEnabled != 0
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.local_fields_enabled() {
            write!(f, " local-fields-enabled")?;
        }
        if self.lp_reliability_enabled() {
            write!(f, " lp-reliability-enabled")?;
        }
        if self.congestion_marking_enabled() {
            write!(f, " congestion-marking-enabled")?;
        }

        Ok(())
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
