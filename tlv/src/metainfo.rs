use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct MetaInfo {
    content_type: Option<ContentType>,
    freshness_period: Option<FreshnessPeriod>,
    final_block_id: Option<FinalBlockId>,
}

impl Tlv for MetaInfo {
    fn r#type(&self) -> Type {
        Type::MetaInfo
    }

    fn value(&self) -> Option<Bytes> {
        todo!()
    }

    fn payload_size(&self) -> usize {
        todo!()
    }
}
