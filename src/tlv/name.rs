use std::fmt;

use bytes::Bytes;
use tlv::Tlv;
use tlv::VarNumber;

use tlv::NameComponent as GenericNameComponent;
use tlv::ImplicitSha256DigestComponent;

#[derive(Debug, PartialEq)]
enum NameComponent {
    GenericNameComponent(GenericNameComponent),
    ImplicitSha256DigestComponent(ImplicitSha256DigestComponent),
}

#[derive(Debug, PartialEq)]
pub struct Name {
    components: Vec<NameComponent>,
    length: VarNumber,
}

impl Name {
    pub fn with_digest() -> Self {
        let digest = ImplicitSha256DigestComponent::new();
        let length = digest.length() + 1 + 1;
        let components = vec![NameComponent::ImplicitSha256DigestComponent(digest)];
        Self { components, length }
    }
}
impl Tlv for Name {
    const TYPE: u64 = 0x07;

    fn length(&self) -> VarNumber {
        self.length
    }

    fn value(&self) -> Option<Bytes> {
        unimplemented!()
    }

    fn size(&self) -> usize {
        unimplemented!()
    }
}

impl fmt::Display for Name {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self.components)
    }
}
