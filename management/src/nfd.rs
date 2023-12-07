use tlv::DecodeError;

use super::*;

#[derive(Debug)]
pub struct GeneralStatus {
    pub version: NfdVersion,
}

impl GeneralStatus {
    const NAME: &'static str = "/localhost/nfd/status/general";
}

impl TryFrom<tlv::Data> for GeneralStatus {
    type Error = tlv::DecodeError;

    fn try_from(data: tlv::Data) -> Result<Self, Self::Error> {
        let mut items = data
            .name_starts_with(Self::NAME)?
            .content
            .ok_or(DecodeError::InvalidData)?
            .into_iter();

        let version = items
            .next()
            .map(NfdVersion::try_from)
            .transpose()?
            .expect("NfdVersion must be present");

        Ok(Self { version })
    }
}

tlv::utf8_string!(NfdVersion => tlv::Type::NfdVersion);
