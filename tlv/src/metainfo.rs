use super::*;

#[derive(Clone, Debug, Default, PartialEq, Tlv)]
#[tlv(r#type = Type::MetaInfo, error = DecodeError)]
pub struct MetaInfo {
    pub content_type: Option<ContentType>,
    pub freshness_period: Option<FreshnessPeriod>,
    pub final_block_id: Option<FinalBlockId>,
}

impl fmt::Display for MetaInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "MetaInfo")?;
        } else {
            write!(f, "{}", self.r#type())?;
        }
        write!(f, "=[")?;
        display_option(&self.content_type, f)?;
        write!(f, " ")?;
        display_option(&self.freshness_period, f)?;
        write!(f, " ")?;
        display_option(&self.final_block_id, f)?;
        write!(f, "]")
    }
}
