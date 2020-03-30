use std::fmt;

use bytes::Bytes;

use super::Tlv;
use super::VarNumber;

#[derive(Debug, PartialEq)]
pub struct MustBeFresh;

impl Tlv for MustBeFresh {
    const TYPE: u64 = 0x12;

    fn length(&self) -> VarNumber {
        0u64.into()
    }

    fn value(&self) -> Option<Bytes> {
        None
    }

    fn size(&self) -> usize {
        unimplemented!()
    }
}

impl fmt::Display for MustBeFresh {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "MustBeFresh")
    }
}
