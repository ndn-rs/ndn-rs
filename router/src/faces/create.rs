use super::*;

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

#[derive(Debug)]
pub(super) struct CreateRequest {
    /// canonical remote FaceUri of the face to create.
    uri: face::Uri,
    /// canonical local FaceUri of the face to create; e.g., FaceUri of the local interface for an Ethernet unicast face.
    local_uri: Option<face::LocalUri>,
    /// either persistent or permanent; creating on-demand faces is not permitted. The default is persistent. See "face properties" for more information.
    face_persistency: Option<face::FacePersistency>,
    base_congestion_marking_interval: Option<mgmt::BaseCongestionMarkingInterval>,
    default_congestion_threshold: Option<mgmt::DefaultCongestionThreshold>,
    mtu: Option<face::Mtu>,
    flags_and_mask: Option<(mgmt::Flags, mgmt::Mask)>,
}

#[derive(Debug)]
pub(super) struct CreateResponse {
    face_id: face::FaceId,
    uri: face::Uri,
    local_uri: face::LocalUri,
    face_persistency: face::FacePersistency,
    base_congestion_marking_interval: Option<mgmt::BaseCongestionMarkingInterval>,
    default_congestion_threshold: Option<mgmt::DefaultCongestionThreshold>,
    mtu: Option<face::Mtu>,
    flags: mgmt::Flags,
}

impl FaceManegement {
    #[tracing::instrument]
    pub(super) async fn create_impl(&self, create: CreateRequest) -> io::Result<CreateResponse> {
        tracing::debug!(?create);
        let persistency = create.face_persistency.unwrap_or_default();
        let face = Face::new(create.uri, create.local_uri, persistency, create.mtu).await?;
        tracing::info!(?face, "CREATED");

        face.update_congestion(
            create.base_congestion_marking_interval,
            create.default_congestion_threshold,
        )
        .await?;

        face.update_flags(create.flags_and_mask).await?;

        let id = self.insert(face).await;
        self.get_face(&id)
            .await
            .map(|face| CreateResponse::from_face(&face))
            .ok_or_else(|| io::Error::other("failed to register"))
    }
}

impl TryFrom<mgmt::ControlParameters> for CreateRequest {
    type Error = &'static str;

    fn try_from(params: mgmt::ControlParameters) -> Result<Self, Self::Error> {
        let mgmt::ControlParameters {
            // name,
            // face_id,
            uri,
            local_uri,
            // origin,
            // cost,
            // capacity,
            // count,
            base_congestion_marking_interval,
            default_congestion_threshold,
            mtu,
            flags,
            mask,
            // strategy,
            // expiration_period,
            face_persistency,
            ..
        } = params;

        let uri = uri.ok_or("Uri is missing")?;

        if face_persistency == Some(face::FacePersistency::OnDemand) {
            Err("creating on-demand faces is not permitted")?;
        }

        let flags_and_mask = match (flags, mask) {
            (None, None) => None,
            (None, Some(_)) => Err("Mask without Flags")?,
            (Some(_), None) => Err("Flags without Mask")?,
            (Some(flags), Some(mask)) => Some((flags, mask)),
        };

        Ok(Self {
            uri,
            local_uri,
            face_persistency,
            base_congestion_marking_interval,
            default_congestion_threshold,
            mtu,
            flags_and_mask,
        })
    }
}

impl CreateResponse {
    fn from_face(face: &Face) -> Self {
        let face_id = face.face_id().clone();
        let uri = face.uri().clone();
        let local_uri = face.local_uri().clone();
        let face_persistency = face.persistency();
        let mtu = Some(face.mtu().clone());
        let flags = mgmt::Flags::from(0); // FIXME

        Self {
            face_id,
            uri,
            local_uri,
            face_persistency,
            base_congestion_marking_interval: None,
            default_congestion_threshold: None,
            mtu,
            flags,
        }
    }
}

impl From<CreateResponse> for mgmt::ControlResponse {
    fn from(response: CreateResponse) -> Self {
        let CreateResponse {
            face_id,
            uri,
            local_uri,
            face_persistency,
            base_congestion_marking_interval,
            default_congestion_threshold,
            mtu,
            flags,
        } = response;

        let body: Vec<Box<dyn tlv::Tlv>> = vec![
            Box::new(face_id),
            Box::new(uri),
            Box::new(local_uri),
            Box::new(face_persistency),
            Box::new(base_congestion_marking_interval),
            Box::new(default_congestion_threshold),
            Box::new(mtu),
            Box::new(flags),
        ];

        let status_code = mgmt::StatusCode::ok();
        let status_text = mgmt::StatusText::from("CREATED");

        Self {
            status_code,
            status_text,
            body,
        }
    }
}
