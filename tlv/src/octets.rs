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

            fn decode_value(
                r#type: $crate::Type,
                length: usize,
                src: &mut bytes::BytesMut,
            ) -> Result<Self, Self::Error> {
                if r#type != $tlv {
                    Err($crate::DecodeError::invalid("TLV-TYPE mismatch"))
                } else if length > src.len() {
                    Err($crate::DecodeError::length_mismatch(length, src.len()))
                } else {
                    Ok(Self(src.split_to(length).freeze()))
                }

                // let _ = (r#type, length);
                // use $crate::TlvCodec;
                // bytes::Bytes::decode(src)
                //     .map(Self)
                //     .map_err($crate::DecodeError::from)
            }
        }

        impl TryFrom<$crate::Generic> for $name {
            type Error = DecodeError;

            fn try_from(generic: $crate::Generic) -> Result<Self, Self::Error> {
                let value = generic
                    .check_type($tlv)?
                    //.self_check_length()?
                    .value
                    .freeze();
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
