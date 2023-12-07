use std::ops;

use super::*;

#[allow(clippy::len_without_is_empty)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonNegativeNumber(pub u64);

impl NonNegativeNumber {
    pub fn to_u64(&self) -> u64 {
        self.0
    }

    pub fn bytes(&self) -> Bytes {
        if let Ok(n) = u8::try_from(self.0) {
            Bytes::copy_from_slice(&n.to_be_bytes())
        } else if let Ok(n) = u16::try_from(self.0) {
            Bytes::copy_from_slice(&n.to_be_bytes())
        } else if let Ok(n) = u32::try_from(self.0) {
            Bytes::copy_from_slice(&n.to_be_bytes())
        } else {
            Bytes::copy_from_slice(&self.0.to_be_bytes())
        }
    }

    pub fn len(&self) -> usize {
        self.bytes().len()
    }
}

impl From<u8> for NonNegativeNumber {
    fn from(value: u8) -> Self {
        Self(u64::from(value))
    }
}

impl From<u16> for NonNegativeNumber {
    fn from(value: u16) -> Self {
        Self(u64::from(value))
    }
}

impl From<u32> for NonNegativeNumber {
    fn from(value: u32) -> Self {
        Self(u64::from(value))
    }
}

impl From<u64> for NonNegativeNumber {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<NonNegativeNumber> for u64 {
    fn from(value: NonNegativeNumber) -> Self {
        value.0
    }
}

impl TryFrom<Bytes> for NonNegativeNumber {
    type Error = DecodeError;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        let mut buf = bytes.as_ref();
        match buf.len() {
            1 => Ok(buf.get_u8().into()),
            2 => Ok(buf.get_u16().into()),
            4 => Ok(buf.get_u32().into()),
            8 => Ok(buf.get_u64().into()),
            _other => Err(DecodeError::InvalidData),
        }
    }
}

impl ops::Deref for NonNegativeNumber {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for NonNegativeNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[macro_export]
macro_rules! non_negative_number {
    ($name: ident => $tlv: expr; skip_display) => {
        $crate::non_negative_number_impl!($name => $tlv);
    };

    ($name: ident => $tlv: expr; prefix => $prefix: literal) => {
        $crate::non_negative_number_impl!($name => $tlv);

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
        $crate::non_negative_number_impl!($name => $tlv);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                format_args!("{}={}", self.r#type(), self.0).fmt(f)
            }
        }
    };
}

#[macro_export]
macro_rules! non_negative_number_impl {
    ($name: ident => $tlv: expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name($crate::NonNegativeNumber);

        impl $name {
            pub fn to_u64(&self) -> u64 {
                self.0.to_u64()
            }

            pub fn to_usize(&self) -> usize {
                self.0.to_u64() as usize
            }
        }

        impl $crate::Tlv for $name {
            fn r#type(&self) -> $crate::Type {
                $tlv
            }

            fn value(&self) -> Option<Bytes> {
                Some(self.0.bytes())
            }

            fn payload_size(&self) -> usize {
                self.0.len()
            }
        }

        impl std::ops::Deref for $name {
            type Target = u64;

            fn deref(&self) -> &Self::Target {
                self.0.deref()
            }
        }

        impl From<u64> for $name {
            fn from(value: u64) -> Self {
                Self($crate::NonNegativeNumber::from(value))
            }
        }

        impl From<$name> for u64 {
            fn from(value: $name) -> Self {
                value.0.into()
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
                    .map($name)
            }
        }
    };
}
