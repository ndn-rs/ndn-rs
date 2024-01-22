//! Face properties as described in https://redmine.named-data.net/projects/nfd/wiki/FaceMgmt#Face-Properties

use super::*;

pub use linktype::LinkType;
pub use persistency::FacePersistency;
pub use scope::FaceScope;

mod linktype;
mod persistency;
mod scope;

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
