use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct MetaInfo {
    pub content_type: Option<ContentType>,
    pub freshness_period: Option<FreshnessPeriod>,
    pub final_block_id: Option<FinalBlockId>,
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

impl fmt::Display for MetaInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.content_type.as_ref().ok_or(fmt::Error)?.fmt(f)
    }
}
