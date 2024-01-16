use super::*;

impl FaceManegement {
    #[tracing::instrument]
    pub(super) async fn destroy_impl(
        &self,
        request: DestroyRequest,
    ) -> io::Result<DestroyResponse> {
        tracing::debug!(?request);
        if let Some(face) = self.remove(request.face_id).await {
            // Teardown face
            tracing::info!(?face, "DESTROYED");
        } else {
            tracing::info!("Face not found; nothing to do");
        }

        Ok(request.into())
    }
}

#[derive(Debug)]
pub(super) struct DestroyRequest {
    face_id: face::FaceId,
}

#[derive(Debug)]
pub(super) struct DestroyResponse {
    face_id: face::FaceId,
}

impl From<DestroyRequest> for DestroyResponse {
    fn from(request: DestroyRequest) -> Self {
        let DestroyRequest { face_id } = request;
        Self { face_id }
    }
}

impl TryFrom<mgmt::ControlParameters> for DestroyRequest {
    type Error = &'static str;

    fn try_from(params: mgmt::ControlParameters) -> Result<Self, Self::Error> {
        let mgmt::ControlParameters {
            // name,
            face_id,
            // uri,
            // local_uri,
            // origin,
            // cost,
            // capacity,
            // count,
            // base_congestion_marking_interval,
            // default_congestion_threshold,
            // mtu,
            // flags,
            // mask,
            // strategy,
            // expiration_period,
            // face_persistency,
            ..
        } = params;

        let face_id = face_id.ok_or("FaceId is missing")?;

        Ok(Self { face_id })
    }
}

impl From<DestroyResponse> for mgmt::ControlResponse {
    fn from(response: DestroyResponse) -> Self {
        Self::face_destroyed(response.face_id)
    }
}
