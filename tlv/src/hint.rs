use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Tlv)]
#[tlv(r#type = Type::ForwardingHint, error = DecodeError)]
pub struct ForwardingHint;

// impl Tlv0 for ForwardingHint {
//     fn r#type(&self) -> Type {
//         Type::ForwardingHint
//     }

//     fn value(&self) -> Option<Bytes> {
//         todo!()
//     }

//     fn payload_size(&self) -> usize {
//         todo!()
//     }
// }

impl fmt::Display for ForwardingHint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "<ForwardingHint>".fmt(f)
    }
}
