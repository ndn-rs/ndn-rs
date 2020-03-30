use std::fmt;

use bytes::Bytes;

use super::Tlv;
use super::VarNumber;

#[derive(Debug, PartialEq)]
pub struct Content {
    bytes: Bytes,
}

impl Tlv for Content {
    const TYPE: u64 = 0x15;

    fn length(&self) -> VarNumber {
        self.bytes.len().into()
    }

    fn value(&self) -> Option<Bytes> {
        Some(self.bytes.clone())
    }

    fn size(&self) -> usize {
        unimplemented!()
    }
}

impl fmt::Display for Content {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "Content({} bytes) <...>", self.bytes.len())
    }
}
