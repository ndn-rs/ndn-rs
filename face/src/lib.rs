use std::fmt;
use std::io;
use std::net;
use std::str;

use bytes::{Buf, BufMut, BytesMut};

// use ndn_packet as packet;
use ndn_tlv as tlv;
// use ndn_varnumber::VarNumber;

pub use congestion::BaseCongestionMarkingInterval;
pub use congestion::DefaultCongestionThreshold;
pub use expiration::ExpirationPeriod;
pub use faceid::FaceId;
pub use flags::Flags;
pub use flags::Mask;
pub use mtu::Mtu;
pub use persistency::FacePersistency;
pub use status::FaceStatus;
pub use uri::Addr;
pub use uri::Internal;
pub use uri::LocalUri;
pub use uri::Tcp;
pub use uri::Udp;
pub use uri::Unix;
pub use uri::Uri;
pub use uri::URI_DELIMITER;

mod congestion;
mod expiration;
mod faceid;
mod flags;
mod mtu;
mod persistency;
mod status;
mod uri;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FaceScope {
    NonLocal = 0,
    Local = 1,
}

// impl Face {
//     pub fn from_boxed(local: Box<dyn FaceUri>, remote: Box<dyn FaceUri>) -> Self {
//         let local = Arc::new(local);
//         let scope = FaceScope::NonLocal;
//         Self {
//             scope,
//             local,
//             remote,
//         }
//     }

//     pub async fn recv(&self) -> io::Result<packet::Packet> {
//         println!("Recv from local {:?}", self.local);
//         let bytes = Bytes::new();
//         Ok(packet::Packet::from_bytes(bytes))
//     }
// }

// impl FaceId {
//     pub async fn send(self, data: Bytes) -> io::Result<()> {
//         // let bytes = data.bytes();
//         println!("Sending {} bytes packet over {:?}", data.len(), self.local);
//         Ok(())
//     }

//     pub async fn recv(&self) -> io::Result<packet::Packet> {
//         self.local.recv().await
//     }
// }

// pub async fn face_uri(uri: impl AsRef<str>) -> Result<Box<dyn FaceUri>, InvalidFaceUri> {
//     let uri = uri.as_ref();
//     if uri.starts_with("udp") {
//         let udp = Udp::from_uri(uri).await?.boxed();
//         Ok(udp)
//     } else if uri.starts_with("tcp") {
//         let tcp = Tcp::from_uri(uri).await?.boxed();
//         Ok(tcp)
//     } else if uri.starts_with("unix") {
//         todo!("unix://path")
//     } else if uri.starts_with("fd") {
//         todo!("fd://<file-descriptor>")
//     } else if uri.starts_with("ether") {
//         todo!("ether://[<MAC>]")
//     } else if uri.starts_with("dev") {
//         todo!("dev://<interface-name>")
//     } else {
//         Err(InvalidFaceUri::new(uri, "unknown scheme"))
//     }
// }
