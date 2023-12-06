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
        pub struct $name(VarNumber);

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
                Self(VarNumber::from(value))
            }
        }

        impl From<$name> for u64 {
            fn from(value: $name) -> Self {
                value.0.into()
            }
        }
    };
}
