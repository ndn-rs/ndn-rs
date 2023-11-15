use super::*;

pub use lifetime::InterestLifetime;

mod lifetime;

#[derive(Clone, Debug, PartialEq)]
pub struct Interest {
    name: Name,
    can_be_prefix: Option<CanBePrefix>,
    must_be_fresh: Option<MustBeFresh>,
    forwarding_hint: Option<ForwardingHint>,
    nonce: Option<Nonce>,
    interest_lifetime: Option<InterestLifetime>,
    hop_limit: Option<HopLimit>,
    application_parameters: Option<ApplicationParameters>,
    interest_signature: Option<InterestSignature>,
}

impl Interest {
    pub fn new(name: impl Into<String>) -> Self {
        let name = Name::generic(name);
        Self {
            name,
            can_be_prefix: None,
            must_be_fresh: None,
            forwarding_hint: None,
            nonce: None,
            interest_lifetime: None,
            hop_limit: None,
            application_parameters: None,
            interest_signature: None,
        }
    }
}

impl Tlv for Interest {
    fn r#type(&self) -> Type {
        Type::Interest
    }

    fn value(&self) -> Option<Bytes> {
        todo!()
    }

    fn payload_size(&self) -> usize {
        todo!()
    }
}

impl fmt::Display for Interest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("Interest<{}>", self.name).fmt(f)
    }
}
