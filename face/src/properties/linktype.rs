use tlv::TlvCodec;

use super::*;

// LinkType indicates the type of communication link.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u64)]
pub enum LinkType {
    /// point-to-point(=0), communication with one peer
    PointToPoint = 0,

    /// multi-access(=1), communication with a multicast group
    MultiAccess = 1,

    /// ad-hoc(=2), communication over a wireless ad hoc network
    AdHoc = 2,
}

impl LinkType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::PointToPoint => "point-to-point",
            Self::MultiAccess => "multi-access",
            Self::AdHoc => "ad-hoc",
        }
    }

    pub fn from_u64(value: u64) -> io::Result<Self> {
        match value {
            0 => Ok(Self::PointToPoint),
            1 => Ok(Self::MultiAccess),
            2 => Ok(Self::AdHoc),
            other => Err(io::Error::other(format!(
                "Invalid LinkType value '{other}'"
            ))),
        }
    }

    pub fn to_u64(&self) -> u64 {
        *self as u64
    }
}

impl From<LinkType> for tlv::NonNegativeNumber {
    fn from(value: LinkType) -> Self {
        value.to_u64().into()
    }
}

impl TryFrom<tlv::NonNegativeNumber> for LinkType {
    type Error = io::Error;

    fn try_from(value: tlv::NonNegativeNumber) -> Result<Self, Self::Error> {
        Self::from_u64(value.0)
    }
}

impl tlv::Tlv for LinkType {
    type Error = tlv::DecodeError;
    const TYPE: tlv::Type = tlv::Type::LinkType;

    fn length(&self) -> usize {
        tlv::NonNegativeNumber::from(*self).total_size()
    }

    fn encode_value(&self, dst: &mut BytesMut) {
        tlv::NonNegativeNumber::from(*self).encode(dst)
    }

    fn decode_value(
        r#type: tlv::Type,
        length: usize,
        src: &mut BytesMut,
    ) -> Result<Self, Self::Error> {
        let _ = (r#type, length);
        tlv::NonNegativeNumber::decode(src)?
            .try_into()
            .map_err(tlv::DecodeError::from)
    }
}

impl fmt::Display for LinkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            self.as_str().fmt(f)
        } else {
            format_args!("{}={}", Self::TYPE, self.as_str()).fmt(f)
        }
    }
}

// // point-to-point(=0), communication with one peer
// // multi-access(=1), communication with a multicast group
// // ad-hoc(=2), communication over a wireless ad hoc network
// tlv::non_negative_number!(LinkType => tlv::Type::LinkType; display_as_str);

// #[allow(non_upper_case_globals)]
// impl LinkType {
//     pub const PointToPoint: Self = Self(tlv::NonNegativeNumber(0));
//     pub const MultiAccess: Self = Self(tlv::NonNegativeNumber(1));
//     pub const AdHoc: Self = Self(tlv::NonNegativeNumber(2));

//     pub fn as_str(&self) -> &'static str {
//         match *self {
//             Self::PointToPoint => "point-to-point",
//             Self::MultiAccess => "multi-access",
//             Self::AdHoc => "as-hoc",
//             _ => "unknown",
//         }
//     }
// }

// impl fmt::Display for LinkType {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         use tlv::Tlv;
//         if f.alternate() {
//             self.as_str().fmt(f)
//         } else {
//             format_args!("{}={}", Self::TYPE, self.as_str()).fmt(f)
//         }
//     }
// }
