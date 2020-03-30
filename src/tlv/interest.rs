use std::fmt;

use bytes::Bytes;

use super::{
    ForwardingHint, ImplicitSha256DigestComponent, InterestLifetime, Name, NameComponent, Nonce,
    Selectors, Tlv, VarNumber,
};

#[derive(Debug, PartialEq)]
pub struct Interest {
    name: Name,
    selectors: Option<Selectors>,
    nonce: Nonce,
    interestlifetime: Option<InterestLifetime>,
    forwardinghint: Option<ForwardingHint>,
}

impl Interest {
    pub fn with_digest() -> Self {
        let digest = ImplicitSha256DigestComponent::new();
        let length = digest.length() + 1 + 1;
        let components = vec![NameComponent::ImplicitSha256DigestComponent(digest)];
        Self { components, length }
    }
}

impl Tlv for Interest {
    const TYPE: u64 = 0x05;

    fn length(&self) -> VarNumber {
        self.length.clone()
    }

    fn value(&self) -> Option<Bytes> {
        unimplemented!()
    }

    fn size(&self) -> usize {
        unimplemented!()
    }
}

impl fmt::Display for Interest {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}", self.components)
    }
}
