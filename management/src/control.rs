use super::*;

#[derive(Clone, Debug, Default)]
pub struct ControlParameters {
    pub name: Option<tlv::Name>,
    pub face_id: Option<face::FaceId>,
    pub uri: Option<face::Uri>,
    pub local_uri: Option<face::LocalUri>,
    pub origin: Option<Origin>,
    pub cost: Option<Cost>,
    pub capacity: Option<Capacity>,
    pub count: Option<Count>,
    pub base_congestion_marking_interval: Option<face::BaseCongestionMarkingInterval>,
    pub default_congestion_threshold: Option<face::DefaultCongestionThreshold>,
    pub mtu: Option<face::Mtu>,
    pub flags: Option<face::Flags>,
    pub mask: Option<face::Mask>,
    pub strategy: Option<Strategy>,
    pub expiration_period: Option<face::ExpirationPeriod>,
    pub face_persistency: Option<face::FacePersistency>,
}

impl ControlParameters {
    // CREATE ControlParameters fields:
    //  Uri (required): canonical remote FaceUri of the face to create.
    //  LocalUri (optional): canonical local FaceUri of the face to create; e.g., FaceUri of the local interface for an Ethernet unicast face.
    //  FacePersistency (optional): either persistent or permanent; creating on-demand faces is not permitted. The default is persistent. See "face properties" for more information.
    //  BaseCongestionMarkingInterval (optional): see "face properties".
    //  DefaultCongestionThreshold (optional): see "face properties".
    //  Mtu (optional): see "face properties".
    //  Flags (optional): see "face properties".
    //  Mask (optional): MUST be specified if Flags is present, and omitted if Flags is omitted.
    // This command allows the creation of UDP unicast, Ethernet unicast, and TCP faces only.

    pub fn create_face(uri: impl Into<face::Uri>) -> Self {
        Self {
            uri: Some(uri.into()),
            ..Self::default()
        }
    }
}

impl tlv::Tlv for ControlParameters {
    fn r#type(&self) -> tlv::Type {
        tlv::Type::ControlParameters
    }

    fn value(&self) -> Option<Bytes> {
        let items = [
            self.name.value(),
            self.face_id.value(),
            self.uri.value(),
            self.local_uri.value(),
            self.origin.value(),
            self.cost.value(),
            self.capacity.value(),
            self.count.value(),
            self.base_congestion_marking_interval.value(),
            self.default_congestion_threshold.value(),
            self.mtu.value(),
            self.flags.value(),
            self.mask.value(),
            self.strategy.value(),
            self.expiration_period.value(),
            self.face_persistency.value(),
        ];
        tlv::collect_to_bytes(items)
    }

    fn payload_size(&self) -> usize {
        todo!()
    }
}
