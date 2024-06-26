use std::io;

// use bytes::Bytes;
use ndn_face as face;
use ndn_tlv as tlv;
// use ndn_varnumber::VarNumber;

// Re-export for uniformity
pub use face::FaceStatus;

pub use capacity::Capacity;
pub use control::ControlParameters;
pub use cost::Cost;
pub use count::Count;
pub use nfd::ChannelStatus;
pub use nfd::GeneralStatus;
pub use origin::Origin;
pub use response::ControlResponse;
pub use status::StatusCode;
pub use status::StatusText;
pub use strategy::Strategy;

mod capacity;
mod control;
mod cost;
mod count;
mod nfd;
mod origin;
mod response;
mod status;
mod strategy;

fn default<T: Default>() -> T {
    T::default()
}
