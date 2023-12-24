use super::*;

#[allow(non_upper_case_globals)]
impl Type {
    pub const Unassigned: Self = Self(0);

    pub const ImplicitSha256DigestComponent: Self = Self(1);
    pub const ParametersSha256DigestComponent: Self = Self(2);
    // 3 Unassigned
    // 4 Unassigned
    pub const Interest: Self = Self(5);
    pub const Data: Self = Self(6);
    pub const Name: Self = Self(7);
    pub const GenericNameComponent: Self = Self(8);
    // 9 (0x09) Reserved (formerly Selectors)
    pub const Nonce: Self = Self(10);
    // 11 (0x0b) Reserved (formerly Scope)
    pub const InterestLifetime: Self = Self(12);
    // 13 (0x0d) Reserved (formerly MinSuffixComponents)
    // 14 (0x0e) Reserved (formerly MaxSuffixComponents)
    // 15 (0x0f) Reserved (formerly PublisherPublicKeyLocator)
    // 16 (0x10) Reserved (formerly Exclude)
    // 17 (0x11) Reserved (formerly ChildSelector)
    pub const MustBeFresh: Self = Self(18);
    // 19 (0x13) Reserved (formerly Any)
    pub const MetaInfo: Self = Self(20);
    pub const Content: Self = Self(21);
    pub const SignatureInfo: Self = Self(22);
    pub const SignatureValue: Self = Self(23);
    pub const ContentType: Self = Self(24);
    pub const FreshnessPeriod: Self = Self(25);
    pub const FinalBlockId: Self = Self(26);
    pub const SignatureType: Self = Self(27);
    pub const KeyLocator: Self = Self(28);
    pub const KeyDigest: Self = Self(29);
    pub const ForwardingHint: Self = Self(30);
    // 31 (0x1f) Reserved (formerly Delegation)
    pub const KeywordNameComponent: Self = Self(32);
    pub const CanBePrefix: Self = Self(33);
    pub const HopLimit: Self = Self(34);
    // 35 0x23 Reserved
    pub const ApplicationParameters: Self = Self(36);
    // 37 0x25 Reserved
    pub const SignatureNonce: Self = Self(38);
    pub const SignatureTime: Self = Self(40);
    pub const SignatureSeqNum: Self = Self(42);
    pub const InterestSignatureInfo: Self = Self(44);
    pub const InterestSignatureValue: Self = Self(46);
    pub const SegmentNameComponent: Self = Self(50);
    pub const ByteOffsetNameComponent: Self = Self(52);
    pub const VersionNameComponent: Self = Self(54);
    pub const TimestampNameComponent: Self = Self(56);
    pub const SequenceNumNameComponent: Self = Self(58);
    pub const Fragment: Self = Self(80);
    pub const Sequence: Self = Self(81);
    pub const FragIndex: Self = Self(82);
    pub const FragCount: Self = Self(83);
    pub const HopCount: Self = Self(84);
    pub const GeoTag: Self = Self(85);
    pub const PitToken: Self = Self(98);
    pub const LpPacket: Self = Self(100);
    pub const ControlResponse: Self = Self(101);
    pub const StatusCode: Self = Self(102);
    pub const StatusText: Self = Self(103);
    pub const ControlParameters: Self = Self(104);
    pub const FaceId: Self = Self(105);
    pub const Cost: Self = Self(106);
    pub const Strategy: Self = Self(107);
    pub const Flags: Self = Self(108);
    pub const ExpirationPeriod: Self = Self(109);
    // 110 (0x6e) reserved, (formerly LocalControlFeature)
    pub const Origin: Self = Self(111);
    pub const Mask: Self = Self(112);
    // 113 reserved
    pub const Uri: Self = Self(114);
    pub const FaceStatus: Self = Self(128);
    pub const NfdVersion: Self = Self(128); // Conflicts with FaceStatus
    pub const LocalUri: Self = Self(129);
    pub const StartTimestamp: Self = Self(129);
    pub const ChannelStatus: Self = Self(130);
    pub const CurrentTimestamp: Self = Self(130);
    pub const Capacity: Self = Self(131);
    pub const UriScheme: Self = Self(131); // conflicts with Capacity
    pub const Count: Self = Self(132);
    pub const FaceScope: Self = Self(132); // conflicts with Count
    pub const FacePersistency: Self = Self(133);
    pub const LinkType: Self = Self(134);
    pub const BaseCongestionMarkingInterval: Self = Self(135);
    pub const DefaultCongestionThreshold: Self = Self(136);
    pub const Mtu: Self = Self(137);
    pub const NInInterests: Self = Self(144);
    pub const NInData: Self = Self(145);
    pub const NOutInterests: Self = Self(146);
    pub const NOutData: Self = Self(147);
    pub const NInBytes: Self = Self(148);
    pub const NOutBytes: Self = Self(149);
    pub const FaceQueryFilter: Self = Self(150);
    pub const NInNacks: Self = Self(151);
    pub const NOutNacks: Self = Self(152);
    pub const FaceEventNotification: Self = Self(192);
    pub const FaceEVentKind: Self = Self(193);
    // 194 (0xc2) (reserved, formerly FaceFlags)
    pub const ValidityPeriod: Self = Self(253);
    pub const NotBefore: Self = Self(254);
    pub const NotAfter: Self = Self(255);
    pub const AdditionalDescription: Self = Self(258);
    pub const DescriptionEntry: Self = Self(512);
    pub const DescriptionKey: Self = Self(513);
    pub const DescriptionValue: Self = Self(514);
    pub const Nack: Self = Self(800);
    pub const NackReason: Self = Self(801);
    pub const IncomingFaceId: Self = Self(812);
    pub const NextHopFaceId: Self = Self(816);
    // 821 0x0331 Reserved, formerly IncomingFaceId
    pub const CachePolicy: Self = Self(820);
    pub const CachePolicyType: Self = Self(821);
    pub const CongestionMark: Self = Self(832);
    pub const Ack: Self = Self(836);
    pub const TxSequence: Self = Self(840);
    pub const NonDiscovery: Self = Self(844);
    pub const PrefixAnnouncement: Self = Self(848);
}
