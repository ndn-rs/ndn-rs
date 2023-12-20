use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Tlv)]
#[tlv(r#type = Type::ApplicationParameters, error = DecodeError)]
pub struct ApplicationParameters;

// impl Tlv0 for ApplicationParameters {
//     fn r#type(&self) -> Type {
//         Type::ApplicationParameters
//     }

//     fn value(&self) -> Option<Bytes> {
//         todo!()
//     }

//     fn payload_size(&self) -> usize {
//         todo!()
//     }
// }

impl fmt::Display for ApplicationParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "<ApplicationParameters>".fmt(f)
    }
}
