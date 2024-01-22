//! Face properties as described in https://redmine.named-data.net/projects/nfd/wiki/FaceMgmt#Face-Properties

use super::*;

pub use persistency::FacePersistency;
pub use scope::FaceScope;

mod persistency;
mod scope;

// LinkType indicates the type of communication link.

// point-to-point(=0), communication with one peer
// multi-access(=1), communication with a multicast group
// ad-hoc(=2), communication over a wireless ad hoc network
tlv::non_negative_number!(LinkType => tlv::Type::LinkType; display_as_str);

#[allow(non_upper_case_globals)]
impl LinkType {
    pub const PointToPoint: Self = Self(tlv::NonNegativeNumber(0));
    pub const MultiAccess: Self = Self(tlv::NonNegativeNumber(1));
    pub const AdHoc: Self = Self(tlv::NonNegativeNumber(2));

    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::PointToPoint => "point-to-point",
            Self::MultiAccess => "multi-access",
            Self::AdHoc => "as-hoc",
            _ => "unknown",
        }
    }
}

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

tlv::non_negative_number!(NInInterests => tlv::Type::NInInterests); // number of incoming Interest packets processed since the forwarder started
tlv::non_negative_number!(NInData => tlv::Type::NInData); // number of incoming Data packets processed since the forwarder started
tlv::non_negative_number!(NInNacks => tlv::Type::NInNacks); // number of incoming Nack packets processed since the forwarder started
tlv::non_negative_number!(NOutInterests => tlv::Type::NOutInterests); // number of outgoing Interest packets processed since the forwarder started
tlv::non_negative_number!(NOutData => tlv::Type::NOutData); // number of outgoing Data packets processed since the forwarder started
tlv::non_negative_number!(NOutNacks => tlv::Type::NOutNacks); // number of outgoing Nack packets processed since the forwarder started

// NInBytes counts the number of bytes of link layer packets received via this face.
// This counter is initialized to zero when the face is established,
// and can wrap around after overflowing unsigned 64-bit integer range.
tlv::non_negative_number!(NInBytes => tlv::Type::NInBytes);

// NOutBytes counts the number of bytes of link layer packets sent via this face.
// This counter is initialized to zero when the face is established,
// and can wrap around after overflowing unsigned 64-bit integer range.
tlv::non_negative_number!(NOutBytes => tlv::Type::NOutBytes);
