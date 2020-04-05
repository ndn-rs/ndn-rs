use std::fmt;

use bytes::Bytes;

use super::{Tlv, VarNumber};

#[derive(Debug, PartialEq)]
pub struct HopLimit {
    limit: u8,
}

impl Tlv for HopLimit {
    const TYPE: u64 = 0x0a;

    fn length(&self) -> VarNumber {
        1_u64.into()
    }

    fn value(&self) -> Option<Bytes> {
        let bytes = Bytes::copy_from_slice(&[self.limit]);
        Some(bytes)
    }

    fn size(&self) -> usize {
        unimplemented!()
    }
}

impl fmt::Display for HopLimit {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "HopLimit <{}>", self.limit)
    }
}
