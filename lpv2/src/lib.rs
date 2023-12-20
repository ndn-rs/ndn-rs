use std::io;

use bytes::{Buf, BufMut, Bytes, BytesMut};

use ndn_tlv as tlv;

pub use fragment::Fragment;
pub use sequence::Sequence;

mod fragment;
mod sequence;

#[derive(Clone, Debug, PartialEq)]
pub struct LpPacket {
    lp_header_field: Vec<Sequence>,
    fragment: Option<Fragment>,
}
