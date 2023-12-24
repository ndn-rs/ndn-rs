use super::*;

#[derive(Clone, Debug, Default, PartialEq, Tlv)]
#[tlv(r#type = Type::MetaInfo, error = DecodeError)]
pub struct MetaInfo {
    pub content_type: Option<ContentType>,
    pub freshness_period: Option<FreshnessPeriod>,
    pub final_block_id: Option<FinalBlockId>,
}

impl TryFrom<Generic> for MetaInfo {
    type Error = DecodeError;

    fn try_from(generic: Generic) -> Result<Self, Self::Error> {
        let items = generic
            .items()
            .ok_or_else(|| DecodeError::other("Empty MetaInfo"))?
            .into_iter();

        let mut content_type = None;
        let mut freshness_period = None;
        let mut final_block_id = None;

        for item in items {
            match item.r#type {
                Type::ContentType => content_type = ContentType::try_from(item)?.into(),
                Type::FreshnessPeriod => freshness_period = FreshnessPeriod::try_from(item)?.into(),
                Type::FinalBlockId => final_block_id = FinalBlockId::try_from(item)?.into(),
                other => {
                    return Err(DecodeError::other(format!(
                        "Unexpected type {other} in MetaInfo"
                    )))
                }
            }
        }
        Ok(Self {
            content_type,
            freshness_period,
            final_block_id,
        })
    }
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
