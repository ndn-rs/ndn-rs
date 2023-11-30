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
}

impl tlv::Tlv for FacePersistency {
    fn r#type(&self) -> tlv::Type {
        tlv::Type::FacePersistency
    }

    fn value(&self) -> Option<Bytes> {
        Some(Bytes::copy_from_slice(&[*self as u8]))
    }

    fn payload_size(&self) -> usize {
        1
    }
}

impl fmt::Display for FacePersistency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

#[cfg(test)]
mod tests {
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
