use super::*;

#[derive(Clone, Debug)]
pub struct FaceStatus {
    pub face_id: FaceId,
    pub uri: Uri,
    pub local_uri: LocalUri,
    pub expiration_period: Option<ExpirationPeriod>,
    pub face_scope: FaceScope,
    pub face_persistency: FacePersistency,
    // FIXME - More data from https://redmine.named-data.net/projects/nfd/wiki/FaceMgmt#Face-Dataset
    // LinkType
    // BaseCongestionMarkingInterval
    // DefaultCongestionThreshold
    pub mtu: Option<Mtu>,
}
