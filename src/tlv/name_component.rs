use std::fmt;

use bytes::Bytes;
use tlv::Tlv;
use tlv::VarNumber;

#[derive(Debug, PartialEq)]
pub struct NameComponent {
    bytes: Bytes,
}

impl Tlv for NameComponent {
    const TYPE: u8 = 0x08;

    fn length(&self) -> VarNumber {
        self.bytes.len().into()
    }

    fn value(&self) -> Option<Bytes> {
        Some(self.bytes.clone())
    }
}

impl fmt::Display for NameComponent {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self.bytes)
    }
}
