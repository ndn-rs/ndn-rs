use super::*;

pub use digest::ImplicitSha256DigestComponent;
pub use digest::ParametersSha256DigestComponent;
pub use generic::GenericNameComponent;
pub use keyword::KeywordNameComponent;
pub use other::ByteOffsetNameComponent;
pub use other::OtherTypeComponent;
pub use other::SegmentNameComponent;
pub use other::SequenceNumNameComponent;
pub use other::TimestampNameComponent;
pub use other::VersionNameComponent;

mod digest;
mod generic;
mod keyword;
mod other;

// KeywordNameComponent	32 (0x20)	*OCTET	Well-known keyword	(not defined)
// SegmentNameComponent	50 (0x32)	NonNegativeInteger	Segment number	seg=<dec>	NDN naming conventions
// ByteOffsetNameComponent	52 (0x34)	NonNegativeInteger	Byte offset	off=<dec>	NDN naming conventions
// VersionNameComponent	54 (0x36)	NonNegativeInteger	Version number	v=<dec>	NDN naming conventions
// TimestampNameComponent	56 (0x38)	NonNegativeInteger	Unix timestamp in microseconds	t=<dec>	NDN naming conventions
// SequenceNumNameComponent	58 (0x3a)	NonNegativeInteger	Sequence number	seq=<dec>	NDN naming conventions

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NameComponent {
    GenericName(GenericNameComponent),
    ImplicitSha256Digest(ImplicitSha256DigestComponent),
    ParametersSha256Digest(ParametersSha256DigestComponent),
    Keyword(KeywordNameComponent),
    Segment(SegmentNameComponent),
    ByteOffset(ByteOffsetNameComponent),
    Version(VersionNameComponent),
    Timestamp(TimestampNameComponent),
    SequenceNum(SequenceNumNameComponent),
    OtherType(OtherTypeComponent),
}

impl NameComponent {
    // pub fn size(&self) -> usize {
    //     match self {
    //         Self::GenericName(c) => c.size(),
    //         Self::ImplicitSha256Digest(c) => c.size(),
    //         Self::ParametersSha256Digest(c) => c.size(),
    //         Self::Keyword(c) => c.size(),
    //         Self::Segment(c) => c.size(),
    //         Self::ByteOffset(c) => c.size(),
    //         Self::Version(c) => c.size(),
    //         Self::Timestamp(c) => c.size(),
    //         Self::SequenceNum(c) => c.size(),
    //         Self::OtherType(c) => c.size(),
    //     }
    // }

    // pub fn payload_size(&self) -> usize {
    //     match self {
    //         Self::GenericName(c) => c.payload_size(),
    //         Self::ImplicitSha256Digest(c) => c.payload_size(),
    //         Self::ParametersSha256Digest(c) => c.payload_size(),
    //         Self::Keyword(c) => c.payload_size(),
    //         Self::Segment(c) => c.payload_size(),
    //         Self::ByteOffset(c) => c.payload_size(),
    //         Self::Version(c) => c.payload_size(),
    //         Self::Timestamp(c) => c.payload_size(),
    //         Self::SequenceNum(c) => c.payload_size(),
    //         Self::OtherType(c) => c.payload_size(),
    //     }
    // }

    // pub fn bytes(&self) -> Bytes {
    //     match self {
    //         Self::GenericName(c) => c.bytes(),
    //         Self::ImplicitSha256Digest(c) => c.bytes(),
    //         Self::ParametersSha256Digest(c) => c.bytes(),
    //         Self::Keyword(c) => c.bytes(),
    //         Self::Segment(c) => c.bytes(),
    //         Self::ByteOffset(c) => c.bytes(),
    //         Self::Version(c) => c.bytes(),
    //         Self::Timestamp(c) => c.bytes(),
    //         Self::SequenceNum(c) => c.bytes(),
    //         Self::OtherType(c) => c.bytes(),
    //     }
    // }

    pub fn generic(text: &str) -> Self {
        Self::GenericName(GenericNameComponent::new(text))
    }

    pub fn implicit(text: &str) -> Result<Self, NameError> {
        text.parse().map(Self::ImplicitSha256Digest)
    }

    pub fn parameters(text: &str) -> Result<Self, NameError> {
        text.parse().map(Self::ParametersSha256Digest)
    }

    pub fn other(prefix: &str, text: &str) -> Result<Self, NameError> {
        OtherTypeComponent::with_prefix(prefix, text).map(Self::OtherType)
    }
}

impl str::FromStr for NameComponent {
    type Err = NameError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if let Some((prefix, text)) = text.split_once('=') {
            match prefix {
                ImplicitSha256DigestComponent::PREFIX
                | ImplicitSha256DigestComponent::PREFIX_NUMERIC => Self::implicit(text),
                ParametersSha256DigestComponent::PREFIX
                | ParametersSha256DigestComponent::PREFIX_NUMERIC => Self::parameters(text),
                prefix => Self::other(prefix, text),
            }
        } else {
            Ok(Self::generic(text))
        }
    }
}

impl From<GenericNameComponent> for NameComponent {
    fn from(value: GenericNameComponent) -> Self {
        Self::GenericName(value)
    }
}

impl From<ImplicitSha256DigestComponent> for NameComponent {
    fn from(value: ImplicitSha256DigestComponent) -> Self {
        Self::ImplicitSha256Digest(value)
    }
}

impl From<ParametersSha256DigestComponent> for NameComponent {
    fn from(value: ParametersSha256DigestComponent) -> Self {
        Self::ParametersSha256Digest(value)
    }
}

impl From<KeywordNameComponent> for NameComponent {
    fn from(value: KeywordNameComponent) -> Self {
        Self::Keyword(value)
    }
}

impl From<SegmentNameComponent> for NameComponent {
    fn from(value: SegmentNameComponent) -> Self {
        Self::Segment(value)
    }
}

impl From<VersionNameComponent> for NameComponent {
    fn from(value: VersionNameComponent) -> Self {
        Self::Version(value)
    }
}

impl From<OtherTypeComponent> for NameComponent {
    fn from(value: OtherTypeComponent) -> Self {
        Self::OtherType(value)
    }
}

impl TryFrom<Generic> for NameComponent {
    type Error = DecodeError;

    fn try_from(generic: Generic) -> Result<Self, Self::Error> {
        let component = match generic.r#type {
            Type::ImplicitSha256DigestComponent => {
                ImplicitSha256DigestComponent::try_from(generic)?.into()
            }
            Type::ParametersSha256DigestComponent => {
                ParametersSha256DigestComponent::try_from(generic)?.into()
            }
            Type::GenericNameComponent => GenericNameComponent::try_from(generic)?.into(),
            Type::KeywordNameComponent => KeywordNameComponent::try_from(generic)?.into(),
            Type::SegmentNameComponent => SegmentNameComponent::try_from(generic)?.into(),
            Type::VersionNameComponent => VersionNameComponent::try_from(generic)?.into(),
            other => todo!("Type {other} unimplemented"),
        };
        Ok(component)
    }
}

impl fmt::Display for NameComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let component = match self {
            Self::GenericName(c) => c.to_string(),
            Self::ImplicitSha256Digest(c) => c.to_string(),
            Self::ParametersSha256Digest(c) => c.to_string(),
            Self::Keyword(c) => c.to_string(),
            Self::Segment(c) => c.to_string(),
            Self::ByteOffset(c) => c.to_string(),
            Self::Version(c) => c.to_string(),
            Self::Timestamp(c) => c.to_string(),
            Self::SequenceNum(c) => c.to_string(),
            Self::OtherType(c) => c.to_string(),
        };
        format_args!("/{component}").fmt(f)
    }
}

impl Tlv for NameComponent {
    type Error = DecodeError;

    fn r#type(&self) -> Type {
        match self {
            Self::GenericName(c) => c.r#type(),
            Self::ImplicitSha256Digest(c) => c.r#type(),
            Self::ParametersSha256Digest(c) => c.r#type(),
            Self::Keyword(c) => c.r#type(),
            Self::Segment(c) => c.r#type(),
            Self::ByteOffset(c) => c.r#type(),
            Self::Version(c) => c.r#type(),
            Self::Timestamp(c) => c.r#type(),
            Self::SequenceNum(c) => c.r#type(),
            Self::OtherType(c) => c.r#type(),
        }
    }

    /// Report TLV-LENGTH as usize
    fn length(&self) -> usize {
        match self {
            Self::GenericName(c) => c.length(),
            Self::ImplicitSha256Digest(c) => c.length(),
            Self::ParametersSha256Digest(c) => c.length(),
            Self::Keyword(c) => c.length(),
            Self::Segment(c) => c.length(),
            Self::ByteOffset(c) => c.length(),
            Self::Version(c) => c.length(),
            Self::Timestamp(c) => c.length(),
            Self::SequenceNum(c) => c.length(),
            Self::OtherType(c) => c.length(),
        }
    }

    /// Encode the value into the supplied buffer
    fn encode_value(&self, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match self {
            Self::GenericName(c) => c.encode(dst),
            Self::ImplicitSha256Digest(c) => c.encode(dst),
            Self::ParametersSha256Digest(c) => c.encode(dst),
            Self::Keyword(c) => c.encode(dst),
            Self::Segment(c) => c.encode(dst),
            Self::ByteOffset(c) => c.encode(dst),
            Self::Version(c) => c.encode(dst),
            Self::Timestamp(c) => c.encode(dst),
            Self::SequenceNum(c) => c.encode(dst),
            Self::OtherType(c) => c.encode(dst),
        }
    }

    /// Decode this object from supplied buffer
    fn decode_value(src: &mut BytesMut) -> Result<Self, Self::Error> {
        let _ = src;
        todo!()
    }
}
