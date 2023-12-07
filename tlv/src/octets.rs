#[macro_export]
macro_rules! octets {
    ($name: ident => $tlv: expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub struct $name(pub Bytes);

        impl $name {
            pub fn new(text: impl AsRef<[u8]>) -> Self {
                let bytes = text.as_ref();
                Self(Bytes::copy_from_slice(bytes))
            }
        }

        impl $crate::Tlv for $name {
            fn r#type(&self) -> $crate::Type {
                $tlv
            }

            fn value(&self) -> Option<Bytes> {
                Some(self.0.clone())
            }

            fn payload_size(&self) -> usize {
                self.0.len()
            }
        }

        impl TryFrom<$crate::Generic> for $name {
            type Error = DecodeError;

            fn try_from(generic: $crate::Generic) -> Result<Self, Self::Error> {
                let value = generic.check_type($tlv)?.self_check_length()?.value;
                Ok(Self(value))
            }
        }

        impl<T: Into<String>> From<T> for $name {
            fn from(text: T) -> Self {
                let text = text.into().into();
                Self(text)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let encoded =
                    percent_encoding::percent_encode(&self.0, percent_encoding::NON_ALPHANUMERIC);
                format_args!("{encoded}").fmt(f)
            }
        }
    };
}
