use std::fmt;

use bytes::Bytes;
use tlv::Tlv;
use tlv::VarNumber;

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
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "MaxSuffixComponents <{}>", self.components)
    }
}
