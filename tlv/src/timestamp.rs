use time::ext::NumericalStdDuration;

use super::*;

/// Timestamp in milliseconds since UNIX EPOCH
#[allow(clippy::len_without_is_empty)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MilliSeconds(pub NonNegativeNumber);

impl MilliSeconds {
    pub fn to_u64(&self) -> u64 {
        self.0.to_u64()
    }

    pub fn to_duration(&self) -> std::time::Duration {
        self.to_u64().std_milliseconds()
    }

    pub fn to_system_time(&self) -> std::time::SystemTime {
        std::time::UNIX_EPOCH + self.to_duration()
    }

    pub fn to_offset_datetime(&self) -> time::OffsetDateTime {
        self.to_system_time().into()
    }

    pub fn to_local_datetime(&self) -> time::OffsetDateTime {
        let datetime = self.to_offset_datetime();
        // println!("{:?}", time::UtcOffset::current_local_offset());
        time::UtcOffset::current_local_offset()
            .map_or(datetime, |offset| datetime.to_offset(offset))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl TlvCodec for MilliSeconds {
    type Error = io::Error;
    const TYPE: Type = Type::Unassigned;

    fn total_size(&self) -> usize {
        self.0.total_size()
    }

    fn encode(&self, dst: &mut BytesMut) {
        self.0.encode(dst)
    }

    fn decode(src: &mut BytesMut) -> Result<Self, Self::Error> {
        NonNegativeNumber::decode(src).map(Self)
    }
}

impl From<u64> for MilliSeconds {
    fn from(value: u64) -> Self {
        Self(NonNegativeNumber::from(value))
    }
}

impl fmt::Display for MilliSeconds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            format_args!("{:?}", self.to_system_time()).fmt(f)
        } else {
            self.0.fmt(f)
        }
    }
}

#[macro_export]
macro_rules! milliseconds {
    ($name: ident => $tlv: expr; skip_display) => {
        $crate::milliseconds_impl!($name => $tlv);
    };

    ($name: ident => $tlv: expr; prefix => $prefix: literal) => {
        $crate::milliseconds_impl!($name => $tlv);

        impl $name {
            pub const PREFIX: &'static str = $prefix;
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                format_args!("{}={}", Self::PREFIX, self.0).fmt(f)
            }
        }
    };

    ($name: ident => $tlv: expr) => {
        $crate::milliseconds_impl!($name => $tlv);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use $crate::Tlv;
                format_args!("{}={}", self.r#type(), self.0).fmt(f)
            }
        }
    };
}

#[macro_export]
macro_rules! milliseconds_impl {
    ($name: ident => $tlv: expr) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name($crate::MilliSeconds);

        impl $name {
            pub fn to_u64(&self) -> u64 {
                self.0.to_u64()
            }

            pub fn to_system_time(&self) -> std::time::SystemTime {
                self.0.to_system_time()
            }

            pub fn to_offset_datetime(&self) -> $crate::time::OffsetDateTime {
                self.0.to_offset_datetime()
            }

            pub fn to_local_datetime(&self) -> $crate::time::OffsetDateTime {
                self.0.to_local_datetime()
            }
        }

        impl $crate::Tlv for $name {
            type Error = $crate::DecodeError;
            const TYPE: $crate::Type = $tlv;

            // fn r#type(&self) -> $crate::Type {
            //     $tlv
            // }

            fn length(&self) -> usize {
                self.0.len()
            }

            fn encode_value(&self, dst: &mut bytes::BytesMut) {
                use $crate::TlvCodec;
                self.0.encode(dst)
            }

            fn decode_value(
                r#type: $crate::Type,
                length: usize,
                src: &mut bytes::BytesMut,
            ) -> Result<Self, Self::Error> {
                use $crate::TlvCodec;
                $crate::MilliSeconds::decode(src)
                    .map(Self)
                    .map_err($crate::DecodeError::from)
            }
        }

        // impl std::ops::Deref for $name {
        //     type Target = u64;

        //     fn deref(&self) -> &Self::Target {
        //         self.0.deref()
        //     }
        // }

        impl From<u64> for $name {
            fn from(value: u64) -> Self {
                Self($crate::MilliSeconds::from(value))
            }
        }

        impl From<$name> for u64 {
            fn from(value: $name) -> Self {
                value.to_u64()
            }
        }

        impl TryFrom<$crate::Generic> for $name {
            type Error = $crate::DecodeError;

            fn try_from(generic: $crate::Generic) -> Result<Self, Self::Error> {
                generic
                    .check_type($tlv)?
                    // .self_check_length()?
                    .value
                    .try_into()
                    .map($crate::MilliSeconds)
                    .map($name)
            }
        }
    };
}
