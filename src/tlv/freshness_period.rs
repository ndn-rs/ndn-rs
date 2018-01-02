use std::fmt;

use bytes::Bytes;
use tlv::Tlv;
use tlv::VarNumber;

#[derive(Debug, PartialEq)]
pub struct FreshnessPeriod {
    period: VarNumber,
}

impl Tlv for FreshnessPeriod {
    const TYPE: u64 = 0x19;

    fn length(&self) -> VarNumber {
        self.period.length().into()
    }

    fn value(&self) -> Option<Bytes> {
        Some(self.period.as_bytes())
    }

    fn size(&self) -> usize {
        unimplemented!()
    }
}

impl fmt::Display for FreshnessPeriod {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "FreshnessPeriod <{} ms>", self.period)
    }
}
