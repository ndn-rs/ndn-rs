use slotmap::Key;
use slotmap::KeyData;
use slotmap::SlotMap;
use tokio::sync::RwLock;
use tokio::sync::RwLockMappedWriteGuard;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;

use super::*;

mod create;

#[derive(Debug)]
pub struct Face {
    face_id: face::FaceId,
    uri: face::Uri,
    local_uri: face::LocalUri,
    mtu: face::Mtu,
    persistency: face::FacePersistency,
    transport: transport::Transport,
}

slotmap::new_key_type! { struct FaceKey; }

impl From<face::FaceId> for FaceKey {
    fn from(face: face::FaceId) -> Self {
        let value = face.to_u64();
        KeyData::from_ffi(value).into()
    }
}

#[derive(Debug, Default)]
pub struct FaceManegement {
    faces: RwLock<SlotMap<FaceKey, Face>>,
}

impl FaceManegement {
    pub fn new() -> Self {
        Self::default()
    }

    #[tracing::instrument]
    pub async fn create(&self, params: mgmt::ControlParameters) -> mgmt::ControlResponse {
        match params.try_into() {
            Ok(create) => self.create_impl(create).await.map_or_else(
                mgmt::ControlResponse::socket_error,
                mgmt::ControlResponse::from,
            ),

            Err(reason) => mgmt::ControlResponse::incorrect_control_parameters(reason),
        }
    }

    pub async fn send_item(&self, face: face::FaceId, item: impl tlv::Tlv) -> io::Result<()> {
        self.get_face_mut(face).await?.send_item(item).await
    }

    pub async fn recv_item(&self, face: face::FaceId) -> io::Result<tlv::Generic> {
        self.get_face_mut(face)
            .await?
            .recv_item()
            .await
            .transpose()
            .unwrap()
    }

    // #[tracing::instrument]
    // pub async fn send(&self, face: &face::FaceId, data: Bytes) -> io::Result<()> {
    //     self.get_face(face).await?.send(data).await
    // }

    // #[tracing::instrument]
    // pub async fn recv(&self, face: &face::FaceId) -> io::Result<Bytes> {
    //     self.get_face(face).await?.recv().await
    // }

    pub async fn get_faces(&self) -> Vec<face::FaceId> {
        self.faces
            .read()
            .await
            .values()
            .map(|face| face.face_id())
            .collect()
    }

    #[tracing::instrument]
    pub async fn get_face(&self, face: face::FaceId) -> io::Result<RwLockReadGuard<'_, Face>> {
        let key = face.into();
        let faces = self.faces.read().await;
        RwLockReadGuard::try_map(faces, |faces| faces.get(key))
            .map_err(|_| io::Error::other("FaceId not found"))
    }

    #[tracing::instrument]
    pub async fn get_face_mut(
        &self,
        face: face::FaceId,
    ) -> io::Result<RwLockMappedWriteGuard<'_, Face>> {
        let key = face.into();
        let faces = self.faces.write().await;
        RwLockWriteGuard::try_map(faces, |faces| faces.get_mut(key))
            .map_err(|_| io::Error::other("FaceId not found"))
    }

    async fn insert(&self, face: Face) -> face::FaceId {
        let key = self.faces.write().await.insert_with_key(|key| {
            let id = key.data().as_ffi();
            let face_id = face::FaceId::from(id);
            face.update_face_id(face_id)
        });
        let id = key.data().as_ffi();
        face::FaceId::from(id)
    }
}

impl Face {
    #[tracing::instrument]
    pub async fn new(
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

        let transport = transport::Transport::new(local, remote).await?;
        let local_uri = transport.local_uri()?;
        let mtu = transport.mtu();
        Ok(Self {
            face_id,
            uri,
            local_uri,
            mtu,
            persistency,
            transport,
        })
    }

    #[tracing::instrument]
    pub async fn update_congestion(
        &self,
        base_congestion_marking_interval: Option<face::BaseCongestionMarkingInterval>,
        default_congestion_threshold: Option<face::DefaultCongestionThreshold>,
    ) -> io::Result<()> {
        tracing::warn!("Not implemented yet");
        Ok(())
    }

    #[tracing::instrument]
    pub async fn update_flags(
        &self,
        flags_and_mask: Option<(face::Flags, face::Mask)>,
    ) -> io::Result<()> {
        tracing::warn!("Not implemented yet");
        Ok(())
    }

    pub fn face_id(&self) -> face::FaceId {
        self.face_id
    }

    pub fn update_face_id(self, face_id: face::FaceId) -> Self {
        Self { face_id, ..self }
    }

    pub fn uri(&self) -> &face::Uri {
        &self.uri
    }

    pub fn local_uri(&self) -> &face::LocalUri {
        &self.local_uri
    }

    pub fn persistency(&self) -> face::FacePersistency {
        self.persistency
    }

    pub fn mtu(&self) -> face::Mtu {
        self.mtu
    }

    #[tracing::instrument(skip_all)]
    pub async fn send_item(&mut self, item: impl tlv::Tlv) -> io::Result<()> {
        tracing::trace!(r#type = %item.r#type(), "Outgoing item");
        self.transport.send_item(item).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn recv_item(&mut self) -> io::Result<Option<tlv::Generic>> {
        self.transport.recv_item().await.inspect(|item| {
            item.as_ref()
                .inspect(|item| tracing::trace!(r#type = %item.r#type(), "Incoming item"));
        })
    }

    // pub(crate) async fn send(&self, bytes: Bytes) -> io::Result<()> {
    //     self.transport.send(bytes).await
    // }

    // pub(crate) async fn recv(&self) -> io::Result<Bytes> {
    //     let bytes: BytesMut = BytesMut::with_capacity(self.mtu.to_usize());
    //     self.transport.recv(bytes).await
    // }

    pub fn to_face_status(&self) -> face::FaceStatus {
        let face_id = self.face_id;
        let uri = self.uri.clone();
        let local_uri = self.local_uri.clone();
        let expiration_period = None;
        let face_scope = face::FaceScope::NonLocal;
        let face_persistency = self.persistency;
        let mtu = Some(self.mtu);

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
