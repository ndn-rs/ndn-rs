use std::fmt;

use bytes::Bytes;
use tlv::Tlv;
use tlv::VarNumber;

#[derive(Debug, PartialEq)]
pub struct MinSuffixComponents {
    components: VarNumber,
}

impl Tlv for MinSuffixComponents {
    const TYPE: u64 = 0x0d;

    fn length(&self) -> VarNumber {
        self.components.length().into()
    }

    fn value(&self) -> Option<Bytes> {
        Some(self.components.as_bytes())
    }

    fn size(&self) -> usize {
        unimplemented!()
    }
}

impl fmt::Display for MinSuffixComponents {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "MinSuffixComponents <{}>", self.components)
    }
}
