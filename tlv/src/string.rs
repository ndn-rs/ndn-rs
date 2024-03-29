#[macro_export]
macro_rules! utf8_string {
    ($name: ident => $tlv: expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl tlv::Tlv for $name {
            fn r#type(&self) -> tlv::Type {
                $tlv
            }

            fn value(&self) -> Option<Bytes> {
                let data = self.0.as_bytes();
                let bytes = Bytes::copy_from_slice(data);
                Some(bytes)
            }

            fn payload_size(&self) -> usize {
                self.0.len()
            }
        }

        impl<S> From<S> for $name
        where
            S: Into<String>,
        {
            fn from(s: S) -> Self {
                Self(s.into())
            }
        }

        // impl From<$name> for String {
        //     fn from(s: $name) -> Self {
        //         s.0
        //     }
        // }

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

        impl $name {
            pub fn into_string(self) -> String {
                self.0
            }
        }
    };
}
