use std::fmt;

use bytes::Bytes;

use super::Tlv;
use super::VarNumber;

#[derive(Debug, PartialEq)]
pub struct MaxSuffixComponents {
    components: VarNumber,
}

impl Tlv for MaxSuffixComponents {
    const TYPE: u64 = 0x0e;

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

impl fmt::Display for MaxSuffixComponents {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "MaxSuffixComponents <{}>", self.components)
    }
}
