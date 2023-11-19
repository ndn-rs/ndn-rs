use std::fmt;

use bytes::Bytes;
use ndn_varnumber::VarNumber;

pub use application::ApplicationParameters;
pub use canbeprefix::CanBePrefix;
pub use component::GenericNameComponent;
pub use component::ImplicitSha256DigestComponent;
pub use component::NameComponent;
pub use component::OtherTypeComponent;
pub use component::ParametersSha256DigestComponent;
pub use content::Content;
pub use content::ContentType;
pub use data::Data;
pub use fresh::FreshnessPeriod;
pub use fresh::MustBeFresh;
pub use hint::ForwardingHint;
pub use hoplimit::HopLimit;
pub use interest::Interest;
pub use interest::InterestLifetime;
pub use metainfo::MetaInfo;
pub use name::FinalBlockId;
pub use name::Name;
pub use nonce::Nonce;
pub use packet::Packet;
pub use signature::DataSignature;
pub use signature::InterestSignature;
pub use signature::InterestSignatureInfo;
pub use signature::InterestSignatureValue;
pub use signature::SignatureInfo;
pub use signature::SignatureValue;

mod application;
mod canbeprefix;
mod component;
mod content;
mod data;
mod fresh;
mod hint;
mod hoplimit;
mod interest;
mod metainfo;
mod name;
mod nonce;
mod packet;
mod signature;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Type {
    // 0 Unassigned
    ImplicitSha256DigestComponent = 0x01,
    ParametersSha256DigestComponent = 0x02,
    // 3 Unassigned
    // 4 Unassigned
    Interest = 0x05,
    Data = 0x06,
    Name = 0x07,
    GenericNameComponent = 0x08,
    // 9 (0x09) Reserved (formerly Selectors)
    Nonce = 0x0a,
    // 11 (0x0b) Reserved (formerly Scope)
    InterestLifetime = 0x0c,
    // 13 (0x0d) Reserved (formerly MinSuffixComponents)
    // 14 (0x0e) Reserved (formerly MaxSuffixComponents)
    // 15 (0x0f) Reserved (formerly PublisherPublicKeyLocator)
    // 16 (0x10) Reserved (formerly Exclude)
    // 17 (0x11) Reserved (formerly ChildSelector)
    MustBeFresh = 0x12,
    // 19 (0x13) Reserved (formerly Any)
    MetaInfo = 0x14,
    Content = 0x15,
    SignatureInfo = 0x16,
    SignatureValue = 0x17,
    ContentType = 0x18,
    FreshnessPeriod = 0x19,
    FinalBlockId = 0x1a,
    SignatureType = 0x1b,
    KeyLocator = 0x1c,
    KeyDigest = 0x1d,
    ForwardingHint = 0x1e,
    // 31 (0x1f) Reserved (formerly Delegation)
    KeywordNameComponent = 0x20,
    CanBePrefix = 0x21,
    HopLimit = 0x22,
    // 35 0x23 Reserved
    ApplicationParameters = 0x24,
    // 37 0x25 Reserved
    SignatureNonce = 0x26,
    SignatureTime = 0x28,
    SignatureSeqNum = 0x2a,
    InterestSignatureInfo = 0x2c,
    InterestSignatureValue = 0x2e,
    Fragment = 0x50,
    Sequence = 0x51,
    FragIndex = 0x52,
    FragCount = 0x53,
    HopCount = 0x54,
    GeoTag = 0x55,
    PitToken = 0x62,
    LpPacket = 0x64,
    Nack = 0x0320,
    NackReason = 0x0321,
    IncomingFaceId = 0x032c,
    NextHopFaceId = 0x0330,
    // 821 0x0331 Reserved, formerly IncomingFaceId
    CachePolicy = 0x0334,
    CachePolicyType = 0x0335,
    CongestionMark = 0x0340,
    Ack = 0x0344,
    TxSequence = 0x0348,
    NonDiscovery = 0x034c,
    PrefixAnnouncement = 0x0350,
    ValidityPeriod = 0xfd,
    NotBefore = 0xfe,
    NotAfter = 0xff,
    AdditionalDescription = 0x0102,
    DescriptionEntry = 0x0200,
    DescriptionKey = 0x0201,
    DescriptionValue = 0x0202,
}

pub trait Tlv {
    // /// Each TLV type has its assigned TLV-TYPE number defined as a constant of type u64
    // const TYPE: Type;

    /// Report this TLV-TYPE as `Type`
    fn r#type(&self) -> Type;

    /// report this TLV-TYPE as a `VarNumber`
    fn type_as_varnumber(&self) -> VarNumber {
        VarNumber::from(self.r#type() as u64)
    }
    /// Report TLV-LENGTH as a `VarNumber`
    fn length(&self) -> VarNumber {
        self.payload_size().into()
    }

    /// Report TLV-VALUE as `Bytes` buffer (if value is present)
    fn value(&self) -> Option<Bytes>;

    /// Report the total size of the packet or TLV element, including the TLV-TYPE and TLV-LENGTH
    fn size(&self) -> usize {
        self.payload_size() + self.type_as_varnumber().length() + self.length().length()
    }

    /// Report the size of the payload if any
    fn payload_size(&self) -> usize;
}
