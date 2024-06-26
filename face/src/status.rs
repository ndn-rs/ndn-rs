use super::*;

#[derive(Clone, Debug, tlv::Tlv)]
#[tlv(r#type = tlv::Type::FaceStatus, error = tlv::DecodeError, crates(tlv_core = "tlv::core"))]
pub struct FaceStatus {
    pub face_id: FaceId,
    pub uri: Uri,
    pub local_uri: LocalUri,
    pub expiration_period: Option<ExpirationPeriod>,
    pub face_scope: FaceScope,
    pub face_persistency: FacePersistency,
    pub link_type: LinkType,
    pub base_congestion_marking_interval: Option<BaseCongestionMarkingInterval>,
    pub default_congestion_threshold: Option<DefaultCongestionThreshold>,
    pub mtu: Option<Mtu>,
    pub n_in_interests: NInInterests,
    pub n_in_data: NInData,
    pub n_in_nacks: NInNacks,
    pub n_out_interests: NOutInterests,
    pub n_out_data: NOutData,
    pub n_out_nacks: NOutNacks,
    pub n_in_bytes: NInBytes,
    pub n_out_bytes: NOutBytes,
    pub flags: Flags,
}

impl FaceStatus {
    pub const NAME: &'static str = "/localhost/nfd/faces/list";

    fn congestion(&self) -> Option<(&BaseCongestionMarkingInterval, &DefaultCongestionThreshold)> {
        self.base_congestion_marking_interval
            .as_ref()
            .zip(self.default_congestion_threshold.as_ref())
    }
}

impl fmt::Display for FaceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.face_id, self.uri, self.local_uri)?;

        if let Some(expiration_period) = self.expiration_period {
            write!(f, "{expiration_period:?}")?;
        }

        if let Some(mtu) = self.mtu {
            write!(f, " {}", mtu)?;
        }

        if let Some((bcmi, dct)) = self.congestion() {
            let bcmi = bcmi.to_std_duration();
            write!(
                f,
                " congestion=[base-marking-interval={bcmi:?} default-threshold={dct:#}]",
            )?;
        }

        write!(
            f,
            " counters=[in=[{:#}i {:#}d {:#}n {:#}B] out=[{:#}i {:#}d {:#}n {:#}B]]",
            self.n_in_interests,
            self.n_in_data,
            self.n_in_nacks,
            self.n_in_bytes,
            self.n_out_interests,
            self.n_out_data,
            self.n_out_nacks,
            self.n_out_bytes,
        )?;
        write!(
            f,
            " {:#} {:#} {}",
            self.link_type, self.face_scope, self.face_persistency
        )?;

        write!(f, "{}", self.flags)?;
        Ok(())
    }
}
