use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Strategy(tlv::Name);

// impl tlv::Tlv0 for Strategy {
//     fn r#type(&self) -> tlv::Type {
//         tlv::Type::Strategy
//     }

//     fn value(&self) -> Option<Bytes> {
//         todo!()
//     }

//     fn payload_size(&self) -> usize {
//         todo!()
//     }
// }
