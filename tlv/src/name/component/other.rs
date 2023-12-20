use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OtherTypeComponent {
    pub r#type: Type,
    pub octets: Bytes,
}

impl OtherTypeComponent {
    pub fn with_prefix(prefix: &str, text: &str) -> Result<Self, NameError> {
        let r#type = prefix.parse().or(Err(NameError::InvalidType))?;
        let octets = Bytes::copy_from_slice(text.as_bytes());

        Ok(Self { r#type, octets })
    }

    pub fn as_keyword(&self) -> Option<KeywordNameComponent> {
        (self.r#type == Type::KeywordNameComponent).then(|| KeywordNameComponent::new(&self.octets))
    }
}

impl Tlv for OtherTypeComponent {
    type Error = DecodeError;

    fn r#type(&self) -> Type {
        self.r#type
    }

    fn length(&self) -> usize {
        self.octets.len()
    }

    fn encode_value(&self, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        self.octets.encode(dst).map_err(Self::Error::from)
    }

    fn decode_value(src: &mut bytes::BytesMut) -> Result<Self, Self::Error> {
        let _ = src;
        todo!("OtherTypeComponents::decode_value is not defined yet")
    }
}

// impl Tlv0 for OtherTypeComponent {
//     fn r#type(&self) -> Type {
//         self.r#type
//     }

//     fn value(&self) -> Option<Bytes> {
//         Some(self.octets.clone())
//     }

//     fn payload_size(&self) -> usize {
//         self.octets.len()
//     }
// }

impl fmt::Display for OtherTypeComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let encoded =
            percent_encoding::percent_encode(&self.octets, percent_encoding::NON_ALPHANUMERIC);
        format_args!("{}={}", self.r#type, encoded).fmt(f)
    }
}

non_negative_number!(SegmentNameComponent => Type::SegmentNameComponent; prefix => "seg");
non_negative_number!(ByteOffsetNameComponent => Type::ByteOffsetNameComponent; prefix => "off");
non_negative_number!(VersionNameComponent => Type::VersionNameComponent; prefix => "v");
non_negative_number!(TimestampNameComponent => Type::TimestampNameComponent; prefix => "t");
non_negative_number!(SequenceNumNameComponent => Type::SequenceNumNameComponent; prefix => "seq");
