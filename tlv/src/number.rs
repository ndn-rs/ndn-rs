use std::ops;

use super::*;

#[allow(clippy::len_without_is_empty)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonNegativeNumber(u64);

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
    };
}
