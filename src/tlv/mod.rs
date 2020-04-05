use bytes::Bytes;

mod any;
mod content;
mod freshness_period;
mod implicit_sha256_digest_component;
// mod interest;
mod interest_lifetime;
mod max_suffix_components;
mod min_suffix_components;
mod must_be_fresh;
mod name;
mod name_component;
mod nonce;
mod varnumber;

pub use self::varnumber::VarNumber;

pub use self::any::Any;
pub use self::content::Content;
pub use self::freshness_period::FreshnessPeriod;
pub use self::implicit_sha256_digest_component::ImplicitSha256DigestComponent;
// pub use self::interest::Interest;
pub use self::interest_lifetime::InterestLifetime;
pub use self::max_suffix_components::MaxSuffixComponents;
pub use self::min_suffix_components::MinSuffixComponents;
pub use self::must_be_fresh::MustBeFresh;
pub use self::name::Name;
pub use self::name_component::NameComponent;
pub use self::nonce::Nonce;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    // 0 Unassigned
    // ImplicitSha256DigestComponent = 0x01,
    // 2 - 4 Unassigned
    Interest = 0x05,
    Data = 0x06,
    // Name = 0x07,
    // NameComponent = 0x08,
    // 9 (0x09) Reserved (formerly Selectors)
    Nonce = 0x0a,
    // 11 (0x0b) Reserved (formerly Scope)
    // InterestLifetime = 0x0c,
    // MinSuffixComponents = 0x0d,
    // MaxSuffixComponents = 0x0e,
    PublisherPublicKeyLocator = 0x0f,
    Exclude = 0x10,
    ChildSelector = 0x11,
    // MustBeFresh = 0x12,
    // Any = 0x13,
    MetaInfo = 0x14,
    // Content = 0x15,
    SignatureInfo = 0x16,
    SignatureValue = 0x17,
    ContentType = 0x18,
    // FreshnessPeriod = 0x19,
    FinalBlockId = 0x1a,
    SignatureType = 0x1b,
    KeyLocator = 0x1c,
    KeyDigest = 0x1d,
    Preference = 0x1e,
    Delegation = 0x1f,
}

pub trait Tlv {
    /// Each TLV type has its assigned TLV-TYPE number defined as a constant of type u64
    const TYPE: u64;

    /// report this TLV-TYPE as a `VarNumber`
    fn r#type(&self) -> VarNumber {
        Self::TYPE.into()
    }

    /// Report TLV-LENGTH as a `VarNumber`
    fn length(&self) -> VarNumber;

    /// Report TLV-VALUE as `Bytes` buffer (if value is present)
    fn value(&self) -> Option<Bytes>;

    /// Report the total size of the packet or TLV element, including the TLV-TYPE and TLV-LENGTH
    fn size(&self) -> usize;
}
