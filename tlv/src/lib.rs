use std::fmt;

use bytes::{Bytes, BytesMut};
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
mod number;
mod packet;
mod signature;
mod string;

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
    ControlResponse = 0x65,
    StatusCode = 0x66,
    StatusText = 0x67,
    ControlParameters = 0x68,
    FaceId = 0x69,
    Cost = 0x6a,
    Strategy = 0x6b,
    Flags = 0x6c,
    ExpirationPeriod = 0x6d,
    // 110 (0x6e) reserved, (formerly LocalControlFeature)
    Origin = 0x6f,
    Mask = 0x70,
    Uri = 0x72,
    FaceStatus = 0x80,
    LocalUri = 0x81,
    ChannelStatus = 0x82,
    Capacity = 0x83,
    // UriScheme = 0x83, // conflicts with Capacity, defined as const
    Count = 0x84,
    // FaceScope = 0x84, // conflicts with Count, defined as const
    FacePersistency = 0x85,
    LinkType = 0x86,
    BaseCongestionMarkingInterval = 0x87,
    DefaultCongestionThreshold = 0x88,
    Mtu = 0x89,
    NInInterests = 0x90,
    NInData = 0x91,
    NOutInterests = 0x92,
    NOutData = 0x93,
    NInBytes = 0x94,
    NOutBytes = 0x95,
    FaceQueryFilter = 0x96,
    NInNacks = 0x97,
    NOutNacks = 0x98,
    FaceEventNotification = 0xc0,
    FaceEVentKind = 0xc1,
    // 192 (0xc2) (reserved, formerly FaceFlags)
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

#[allow(non_upper_case_globals)]
impl Type {
    pub const UriScheme: Self = Self::Capacity;
    pub const FaceScope: Self = Self::Count;
}

pub trait Tlv: fmt::Debug {
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
        self.payload_size() + self.type_as_varnumber().len() + self.length().len()
    }

    /// Report the size of the payload if any
    fn payload_size(&self) -> usize;

    /// Convert this TLV to `Bytes`
    fn bytes(&self) -> Bytes {
        let r#type = self.type_as_varnumber().bytes();
        let length = self.length().bytes();
        let payload = self.value().unwrap_or_default();
        let size = r#type.len() + length.len() + payload.len();
        let mut bytes = BytesMut::with_capacity(size);
        bytes.extend([r#type, length, payload]);
        bytes.freeze()
    }
}

impl<T: Tlv> Tlv for Option<T> {
    fn r#type(&self) -> Type {
        self.as_ref()
            .expect("Cannot call .r#type() on None")
            .r#type()

        // self.as_ref()
        //     .map(|t| t.r#type())
        //     .expect("Cannot call .r#type() on None")
    }

    fn value(&self) -> Option<Bytes> {
        self.as_ref().and_then(|t| t.value())
    }

    fn payload_size(&self) -> usize {
        self.as_ref().map(|t| t.payload_size()).unwrap_or_default()
    }
}

pub fn collect_to_bytes<I, O>(items: I) -> Option<Bytes>
where
    I: IntoIterator<Item = O>,
    O: Into<Option<Bytes>>,
{
    let items = items.into_iter().filter_map(|item| item.into());
    let mut bytes = BytesMut::new();
    bytes.extend(items);
    if bytes.is_empty() {
        None
    } else {
        Some(bytes.freeze())
    }
}
