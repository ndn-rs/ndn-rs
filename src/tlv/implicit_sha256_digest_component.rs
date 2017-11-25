use std::fmt;

use bytes::Bytes;
use tlv::Tlv;
use tlv::VarNumber;

#[derive(Debug, PartialEq)]
pub struct ImplicitSha256DigestComponent {
    value: [u8; 32],
}

impl Tlv for ImplicitSha256DigestComponent {
    const TYPE: u8 = 0x01;

    fn length(&self) -> VarNumber {
        32u8.into()
    }

    fn value(&self) -> Option<Bytes> {
        Some(Bytes::from(&self.value[..]))
    }
}

impl fmt::Display for ImplicitSha256DigestComponent {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "sha256digest={:?}", self.value)
    }
}
