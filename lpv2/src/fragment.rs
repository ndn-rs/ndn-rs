use super::*;

#[derive(Clone, Debug, PartialEq, tlv::Tlv)]
#[tlv(r#type = tlv::Type::Fragment, error = tlv::DecodeError, crates(tlv_core = tlv::core))]
pub struct Fragment {
    fragment: Bytes,
}

// impl tlv::Tlv0 for Fragment {
//     fn r#type(&self) -> tlv::Type {
//         tlv::Type::Fragment
//     }

//     fn value(&self) -> Option<Bytes> {
//         Some(self.fragment.clone())
//     }

//     fn payload_size(&self) -> usize {
//         self.fragment.len()
//     }
// }
