use std::fmt;

use bytes::Bytes;
use tlv::Tlv;
use tlv::VarNumber;

#[derive(Debug, PartialEq)]
pub struct Nonce {
    bytes: [u8; 4],
}

impl Tlv for Nonce {
    const TYPE: u64 = 0x0a;

    fn length(&self) -> VarNumber {
        self.bytes.len().into()
    }

    fn value(&self) -> Option<Bytes> {
        Some(self.bytes.as_ref().into())
    }

    fn size(&self) -> usize {
        unimplemented!()
    }
}

impl fmt::Display for Nonce {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "Nonce <{}:{}:{}:{}>",
            self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3]
        )
    }
}
