use tlv::TlvCodec;

use super::*;

// #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, tlv::Tlv)]
// #[tlv(r#type = tlv::Type::FacePersistency, error = tlv::DecodeError, crates(tlv_core = "tlv::core"))]
// pub enum Persistency {
//     /// face remains open until it's explicitly destroyed or there's a transport failure
//     #[default]
//     Persistent = 0,
//     /// face closes if it remains idle for some time
//     OnDemand = 1,
//     /// face remains open until it's explicitly destroyed; transport failures will be recovered internally
//     Permanent = 2,
// }

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u64)]
pub enum FacePersistency {
    /// face remains open until it's explicitly destroyed or there's a transport failure
    #[default]
    Persistent = 0,
    /// face closes if it remains idle for some time
    OnDemand = 1,
    /// face remains open until it's explicitly destroyed; transport failures will be recovered internally
    Permanent = 2,
}

impl FacePersistency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Persistent => "persistent",
            Self::OnDemand => "on-demand",
            Self::Permanent => "permanent",
        }
    }

    pub fn from_u64(value: u64) -> io::Result<Self> {
        match value {
            0 => Ok(Self::Persistent),
            1 => Ok(Self::OnDemand),
            2 => Ok(Self::Permanent),
            other => Err(io::Error::other(format!(
                "Invalid FacePersistency value '{other}'"
            ))),
        }
    }

    pub fn to_u64(&self) -> u64 {
        *self as u64
    }
}

impl From<FacePersistency> for tlv::NonNegativeNumber {
    fn from(value: FacePersistency) -> Self {
        value.to_u64().into()
    }
}

impl TryFrom<tlv::NonNegativeNumber> for FacePersistency {
    type Error = io::Error;

    fn try_from(value: tlv::NonNegativeNumber) -> Result<Self, Self::Error> {
        Self::from_u64(value.0)
    }
}

impl tlv::Tlv for FacePersistency {
    type Error = tlv::DecodeError;
    const TYPE: tlv::Type = tlv::Type::FacePersistency;

    fn length(&self) -> usize {
        tlv::NonNegativeNumber::from(*self).total_size()
    }

    fn encode_value(&self, dst: &mut BytesMut) {
        tlv::NonNegativeNumber::from(*self).encode(dst)
    }

    fn decode_value(
        r#type: tlv::Type,
        length: usize,
        src: &mut BytesMut,
    ) -> Result<Self, Self::Error> {
        let _ = (r#type, length);
        tlv::NonNegativeNumber::decode(src)?
            .try_into()
            .map_err(tlv::DecodeError::from)
    }
}

impl fmt::Display for FacePersistency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use tlv::Tlv;

    use super::*;

    #[test]
    fn on_demand() {
        let fp = FacePersistency::OnDemand;
        let value = fp.value().unwrap();
        assert_eq!(value, Bytes::from_static(&[1]));
    }

    #[test]
    fn persistent() {
        let fp = FacePersistency::Persistent;
        let value = fp.value().unwrap();
        assert_eq!(value, Bytes::from_static(&[0]));
    }

    #[test]
    fn permanent() {
        let fp = FacePersistency::Permanent;
        let value = fp.value().unwrap();
        assert_eq!(value, Bytes::from_static(&[2]));
    }
}
