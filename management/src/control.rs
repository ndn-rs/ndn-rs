use super::*;

#[derive(Clone, Debug, Default, tlv::Tlv)]
#[tlv(r#type = tlv::Type::ControlParameters, error = tlv::DecodeError, crates(tlv_core = "tlv::core"))]
pub struct ControlParameters {
    // pub name: Option<tlv::Name>,
    pub face_id: Option<face::FaceId>,
    pub uri: Option<face::Uri>,
    pub local_uri: Option<face::LocalUri>,
    // pub origin: Option<Origin>,
    // pub cost: Option<Cost>,
    pub capacity: Option<Capacity>,
    pub count: Option<Count>,
    pub base_congestion_marking_interval: Option<face::BaseCongestionMarkingInterval>,
    pub default_congestion_threshold: Option<face::DefaultCongestionThreshold>,
    pub mtu: Option<face::Mtu>,
    pub flags: Option<face::Flags>,
    pub mask: Option<face::Mask>,
    // pub strategy: Option<Strategy>,
    // pub expiration_period: Option<face::ExpirationPeriod>,
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
            ..default()
        }
    }

    // DESTROY ControlParameters fields:
    //  FaceId (required)
    pub fn destroy_face(face: face::FaceId) -> Self {
        Self {
            face_id: Some(face),
            ..default()
        }
    }

    pub fn mtu(self, mtu: impl Into<face::Mtu>) -> Self {
        Self {
            mtu: Some(mtu.into()),
            ..self
        }
    }
}
