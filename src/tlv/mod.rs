use bytes::Bytes;

mod content;
mod freshness_period;
mod hoplimit;
mod implicit_sha256_digest_component;
// mod interest;
mod interest_lifetime;
mod must_be_fresh;
mod name;
mod name_component;
mod nonce;
mod varnumber;

pub use self::varnumber::VarNumber;

pub use self::content::Content;
pub use self::freshness_period::FreshnessPeriod;
pub use self::implicit_sha256_digest_component::ImplicitSha256DigestComponent;
// pub use self::interest::Interest;
pub use self::hoplimit::HopLimit;
pub use self::interest_lifetime::InterestLifetime;
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
    // 13 (0x0d) Reserved (formerly MinSuffixComponents)
    // 14 (0x0e) Reserved (formerly MaxSuffixComponents)
    // 15 (0x0f) Reserved (formerly PublisherPublicKeyLocator)
    // 16 (0x10) Reserved (formerly Exclude)
    // 17 (0x11) Reserved (formerly ChildSelector)
    // MustBeFresh = 0x12,
    // 19 (0x13) Reserved (formerly Any)
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
    // CanBePrefix = 0x21,
    HopLimit = 0x22,
    // ApplicationParameters = 0x24,
    // SignatureNonce = 0x26,
    // SignatureTime = 0x28,
    // SignatureSeqNum = 0x2a,
    // InterestSignatureInfo = 0x2c,
    // InterestSignatureValue = 0x2e,
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
