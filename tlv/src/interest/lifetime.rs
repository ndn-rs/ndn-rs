use super::*;

non_negative_number!(InterestLifetime => Type::InterestLifetime);

// #[derive(Clone, Debug, PartialEq, Eq, Hash)]
// pub struct InterestLifetime;

// impl Tlv for InterestLifetime {
//     fn r#type(&self) -> Type {
//         Type::InterestLifetime
//     }

//     fn value(&self) -> Option<Bytes> {
//         todo!()
//     }

//     fn payload_size(&self) -> usize {
//         todo!()
//     }
// }

// impl fmt::Display for InterestLifetime {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         "<InterestLifetime>".fmt(f)
//     }
// }
