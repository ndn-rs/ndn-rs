use std::fmt;

use bytes::Bytes;

use super::Tlv;
use super::VarNumber;

#[derive(Debug, PartialEq)]
pub struct InterestLifetime {
    lifetime: VarNumber,
}

impl Tlv for InterestLifetime {
    const TYPE: u64 = 0x0c;

    fn length(&self) -> VarNumber {
        self.lifetime.length().into()
    }

    fn value(&self) -> Option<Bytes> {
        Some(self.lifetime.as_bytes())
    }

    fn size(&self) -> usize {
        unimplemented!()
    }
}

impl fmt::Display for InterestLifetime {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "InterestLifetime <{} ms>", self.lifetime)
    }
}
