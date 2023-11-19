use bytes::Bytes;

use ndn_tlv as tlv;

pub use fragment::Fragment;
pub use sequence::Sequence;

mod fragment;
mod sequence;

#[derive(Clone, Debug, PartialEq)]
pub struct LpPacket {
    lp_header_field: Vec<Sequence>,
    fragment: Option<Fragment>,
}

impl tlv::Tlv for LpPacket {
    fn r#type(&self) -> tlv::Type {
        tlv::Type::LpPacket
    }

    fn value(&self) -> Option<Bytes> {
        todo!()
    }

    fn payload_size(&self) -> usize {
        self.lp_header_field
            .iter()
            .map(|item| item.payload_size())
            .chain(self.fragment.iter().map(|item| item.payload_size()))
            .sum()
    }
}
