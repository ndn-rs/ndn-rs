use super::*;

use socket::Socket;

mod socket;

#[derive(Debug)]
pub(crate) struct Face {
    face_id: face::FaceId,
    uri: face::Uri,
    local_uri: face::LocalUri,
    mtu: face::Mtu,
    persistency: face::FacePersistency,
    socket: Socket,
}

impl Face {
    #[tracing::instrument]
    pub(crate) async fn new(
        uri: face::Uri,
        local_uri: Option<face::LocalUri>,
        persistency: face::FacePersistency,
        mtu: Option<face::Mtu>,
    ) -> io::Result<Self> {
        let face_id = face::FaceId::null(); // To be updated with actual FaceId later
        let remote = uri.to_addr().await?;
        let local = if let Some(uri) = local_uri {
            uri.to_addr().await?
        } else {
            remote.any()
        };

        let socket = Socket::new(local, remote).await?;
        let local_uri = socket.local()?;
        let mtu = socket.mtu();
        Ok(Self {
            face_id,
            uri,
            local_uri,
            mtu,
            persistency,
            socket,
        })
    }

    #[tracing::instrument]
    pub(crate) async fn update_congestion(
        &self,
        base_congestion_marking_interval: Option<face::BaseCongestionMarkingInterval>,
        default_congestion_threshold: Option<face::DefaultCongestionThreshold>,
    ) -> io::Result<()> {
        tracing::warn!("Not implemented yet");
        Ok(())
    }

    #[tracing::instrument]
    pub(crate) async fn update_flags(
        &self,
        flags_and_mask: Option<(face::Flags, face::Mask)>,
    ) -> io::Result<()> {
        tracing::warn!("Not implemented yet");
        Ok(())
    }

    pub(crate) fn face_id(&self) -> &face::FaceId {
        &self.face_id
    }

    pub(crate) fn update_face_id(self, face_id: face::FaceId) -> Self {
        Self { face_id, ..self }
    }

    pub(crate) fn uri(&self) -> &face::Uri {
        &self.uri
    }

    pub(crate) fn local_uri(&self) -> &face::LocalUri {
        &self.local_uri
    }

    pub(crate) fn persistency(&self) -> face::FacePersistency {
        self.persistency
    }

    pub(crate) fn mtu(&self) -> &face::Mtu {
        &self.mtu
    }

    pub(crate) async fn send(&self, bytes: Bytes) -> io::Result<()> {
        self.socket.send(bytes).await
    }

    pub(crate) async fn recv(&self) -> io::Result<Bytes> {
        self.socket.recv().await
    }

    pub(crate) fn to_face_status(&self) -> face::FaceStatus {
        let face_id = self.face_id.clone();
        let uri = self.uri.clone();
        let local_uri = self.local_uri.clone();
        let expiration_period = None;
        let face_scope = face::FaceScope::NonLocal;
        let face_persistency = self.persistency;
        let mtu = Some(self.mtu.clone());

        face::FaceStatus {
            face_id,
            uri,
            local_uri,
            expiration_period,
            face_scope,
            face_persistency,
            mtu,
        }
    }
}
