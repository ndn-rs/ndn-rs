use std::fmt;

use bytes::Bytes;

use super::Tlv;
use super::VarNumber;

#[derive(Debug, Default, PartialEq)]
pub struct ImplicitSha256DigestComponent {
    value: [u8; 32],
}

impl ImplicitSha256DigestComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Tlv for ImplicitSha256DigestComponent {
    const TYPE: u64 = 0x01;

    fn length(&self) -> VarNumber {
        self.value.len().into()
    }

    fn value(&self) -> Option<Bytes> {
        let bytes = Bytes::copy_from_slice(&self.value);
        Some(bytes)
    }

    fn size(&self) -> usize {
        1 + 1 + 32
    }
}

impl fmt::Display for ImplicitSha256DigestComponent {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "sha256digest={:?}", self.value)
    }
}
