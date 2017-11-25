use std::fmt;

use bytes::Bytes;
use tlv::Tlv;
use tlv::VarNumber;

#[derive(Debug, Default, PartialEq)]
pub struct ImplicitSha256DigestComponent {
    value: [u8; 32],
}

impl ImplicitSha256DigestComponent {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Tlv for ImplicitSha256DigestComponent {
    const TYPE: u64 = 0x01;

    fn length(&self) -> VarNumber {
        32u64.into()
    }

    fn value(&self) -> Option<Bytes> {
        Some(Bytes::from(&self.value[..]))
    }

    fn size(&self) -> usize {
        1 + 1 + 32
    }
}

impl fmt::Display for ImplicitSha256DigestComponent {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "sha256digest={:?}", self.value)
    }
}
