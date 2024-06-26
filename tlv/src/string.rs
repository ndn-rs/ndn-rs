#[macro_export]
macro_rules! utf8_string {
    ($name: ident => $tlv: expr) => {
        $crate::utf8_string!($name => $tlv; skip_display);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use $crate::Tlv;
                if f.alternate() {
                    format_args!("{}", self.0).fmt(f)
                } else {
                    format_args!("{}={}", self.r#type(), self.0).fmt(f)
                }
            }
        }
    };

    ($name: ident => $tlv: expr; prefix => $prefix: literal) => {
        $crate::utf8_string!($name => $tlv; skip_display);

        impl $name {
            pub const PREFIX: &'static str = $prefix;
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if f.alternate() {
                    format_args!("{}", self.0).fmt(f)
                } else {
                    format_args!("{}={}", Self::PREFIX, self.0).fmt(f)
                }
            }
        }
    };

    ($name: ident => $tlv: expr; skip_display) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub String);

        impl $crate::Tlv for $name {
            type Error = $crate::DecodeError;
            const TYPE: $crate::Type = $tlv;

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
                String::decode(src)
                    .map(Self)
                    .map_err($crate::DecodeError::from)
            }
        }

        impl From<String> for $name {
            fn from(text: String) -> Self {
                Self(text)
            }
        }

        impl From<&str> for $name {
            fn from(text: &str) -> Self {
                Self(text.into())
            }
        }

        impl std::ops::Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                self.0.deref()
            }
        }

        impl std::convert::AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.0.as_ref()
            }
        }

        impl TryFrom<$crate::Generic> for $name {
            type Error = $crate::DecodeError;

            fn try_from(generic: $crate::Generic) -> Result<Self, Self::Error> {
                let bytes = generic
                    .check_type($tlv)?
                    // .self_check_length()?
                    .value
                    .to_vec();
                String::from_utf8(bytes)
                    .map(Self)
                    .map_err(|err| $crate::DecodeError::invalid(err.to_string()))
            }
        }

        impl $name {
            pub fn new(text: impl ToString) -> Self {
                Self(text.to_string())
            }

            pub fn into_string(self) -> String {
                self.0
            }
        }
    };
}
