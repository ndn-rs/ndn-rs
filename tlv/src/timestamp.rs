use super::*;

/// Timestamp in milliseconds since UNIX EPOCH
#[allow(clippy::len_without_is_empty)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MilliSeconds(pub NonNegativeNumber);

impl MilliSeconds {
    pub fn to_u64(&self) -> u64 {
        self.0.to_u64()
    }

    pub fn to_duration(&self) -> time::Duration {
        let millis = self.to_u64();
        time::Duration::from_millis(millis)
    }

    pub fn to_system_time(&self) -> time::SystemTime {
        time::UNIX_EPOCH + self.to_duration()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl TlvCodec for MilliSeconds {
    type Error = io::Error;

    fn total_size(&self) -> usize {
        self.0.total_size()
    }

    fn encode(&self, dst: &mut BytesMut) -> Result<(), Self::Error> {
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
        }

        impl $crate::Tlv for $name {
            type Error = $crate::DecodeError;

            fn r#type(&self) -> $crate::Type {
                $tlv
            }

            fn length(&self) -> usize {
                self.0.len()
            }

            fn encode_value(&self, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
                use $crate::TlvCodec;
                self.0.encode(dst).map_err(Self::Error::from)
            }

            fn decode_value(src: &mut bytes::BytesMut) -> Result<Self, Self::Error> {
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
                    .self_check_length()?
                    .value
                    .try_into()
                    .map($crate::MilliSeconds)
                    .map($name)
            }
        }
    };
}
