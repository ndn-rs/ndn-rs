use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
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

    pub fn from_u8(value: u8) -> io::Result<Self> {
        match value {
            0 => Ok(Self::Persistent),
            1 => Ok(Self::OnDemand),
            2 => Ok(Self::Permanent),
            other => Err(io::Error::other(format!(
                "Invalid FacePersistency value '{other}'"
            ))),
        }
    }
}

impl TryFrom<u8> for FacePersistency {
    type Error = io::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value)
    }
}

impl tlv::Tlv for FacePersistency {
    type Error = tlv::DecodeError;

    fn r#type(&self) -> tlv::Type {
        tlv::Type::FacePersistency
    }

    fn length(&self) -> usize {
        1
    }

    fn encode_value(&self, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_u8(*self as u8);
        Ok(())
    }

    fn decode_value(
        r#type: tlv::Type,
        length: usize,
        src: &mut BytesMut,
    ) -> Result<Self, Self::Error> {
        let _ = (r#type, length);
        let value = src.get_u8();
        Self::from_u8(value).map_err(Self::Error::from)
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
