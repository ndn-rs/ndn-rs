#[macro_export]
macro_rules! non_negative_number {
    ($name: ident => $tlv: expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(VarNumber);

        impl tlv::Tlv for $name {
            fn r#type(&self) -> tlv::Type {
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
