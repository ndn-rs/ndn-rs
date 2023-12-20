use super::*;

pub use lifetime::InterestLifetime;

mod lifetime;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Tlv)]
#[tlv(r#type = Type::Interest, error = DecodeError)]
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
    pub fn new(name: impl AsRef<str>) -> Self {
        let name = name.as_ref().parse().expect("Valid Name");
        let nonce = Some(Nonce::generate());
        // let interest_lifetime = Some(InterestLifetime::from(4_000));
        Self {
            name,
            can_be_prefix: None,
            must_be_fresh: None,
            forwarding_hint: None,
            nonce,
            interest_lifetime: None,
            hop_limit: None,
            application_parameters: None,
            interest_signature: None,
        }
    }

    pub fn must_be_fresh(self) -> Self {
        let must_be_fresh = Some(MustBeFresh);
        Self {
            must_be_fresh,
            ..self
        }
    }

    pub fn can_be_prefix(self) -> Self {
        let can_be_prefix = Some(CanBePrefix);
        Self {
            can_be_prefix,
            ..self
        }
    }
}

// impl Tlv0 for Interest {
//     fn r#type(&self) -> Type {
//         Type::Interest
//     }

//     fn value(&self) -> Option<Bytes> {
//         let items = [
//             self.name.bytes(),
//             self.can_be_prefix.bytes(),
//             self.must_be_fresh.bytes(),
//             self.forwarding_hint.bytes(),
//             self.nonce.bytes(),
//             self.interest_lifetime.bytes(),
//             self.hop_limit.bytes(),
//             self.application_parameters.bytes(),
//             self.interest_signature.bytes(),
//         ];
//         collect_to_bytes(items)
//     }

//     fn payload_size(&self) -> usize {
//         self.name.size()
//             + self.can_be_prefix.size()
//             + self.must_be_fresh.size()
//             + self.forwarding_hint.size()
//             + self.nonce.size()
//             + self.interest_lifetime.size()
//             + self.hop_limit.size()
//             + self.application_parameters.size()
//             + self.interest_signature.size()
//     }
// }

impl fmt::Display for Interest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_args!("interest={}", self.name).fmt(f)?;
        display_option(&self.can_be_prefix, f)?;
        display_option(&self.must_be_fresh, f)?;
        display_option(&self.forwarding_hint, f)?;
        display_option(&self.nonce, f)?;
        display_option(&self.interest_lifetime, f)?;
        display_option(&self.hop_limit, f)?;
        display_option(&self.interest_signature, f)?;
        Ok(())
    }
}
