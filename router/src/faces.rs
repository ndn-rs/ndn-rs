use slotmap::Key;
use slotmap::KeyData;
use slotmap::SlotMap;
use tokio::sync::RwLock;
use tokio::sync::RwLockMappedWriteGuard;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;

use super::*;

mod create;
mod destroy;

#[derive(Debug)]
pub struct Face {
    face_id: face::FaceId,
    uri: face::Uri,
    local_uri: face::LocalUri,
    mtu: face::Mtu,
    persistency: face::FacePersistency,
    base_congestion_marking_interval: Option<face::BaseCongestionMarkingInterval>,
    default_congestion_threshold: Option<face::DefaultCongestionThreshold>,
    flags: face::Flags,
    mask: face::Mask,
    n_in_interests: u64,
    n_in_data: u64,
    n_in_nacks: u64,
    n_out_interests: u64,
    n_out_data: u64,
    n_out_nacks: u64,
    n_in_bytes: u64,
    n_out_bytes: u64,
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

    pub async fn destroy(&self, params: mgmt::ControlParameters) -> mgmt::ControlResponse {
        match params.try_into() {
            Ok(destroy) => self.destroy_impl(destroy).await.map_or_else(
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
        loop {
            if let Some(item) = self.get_face_mut(face).await?.recv_item().await.transpose() {
                break item;
            } else {
                continue;
            }
        }
    }

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

    async fn remove(&self, face: face::FaceId) -> Option<Face> {
        let key = face.into();
        self.faces.write().await.remove(key)
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
        let flags = face::Flags::empty();
        let mask = face::Mask::empty();

        Ok(Self {
            face_id,
            uri,
            local_uri,
            mtu,
            persistency,
            base_congestion_marking_interval: None,
            default_congestion_threshold: None,
            flags,
            mask,
            n_in_interests: 0,
            n_in_data: 0,
            n_in_nacks: 0,
            n_out_interests: 0,
            n_out_data: 0,
            n_out_nacks: 0,
            n_in_bytes: 0,
            n_out_bytes: 0,
            transport,
        })
    }

    #[tracing::instrument]
    pub fn update_congestion(
        self,
        base_congestion_marking_interval: Option<face::BaseCongestionMarkingInterval>,
        default_congestion_threshold: Option<face::DefaultCongestionThreshold>,
    ) -> Self {
        Self {
            base_congestion_marking_interval,
            default_congestion_threshold,
            ..self
        }
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

    pub fn flags(&self) -> face::Flags {
        (*self.flags & *face::Flags::all_fields() & *self.mask).into()
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

    pub fn to_face_status(&self) -> face::FaceStatus {
        let face_id = self.face_id;
        let uri = self.uri.clone();
        let local_uri = self.local_uri.clone();
        let expiration_period = None;
        let face_scope = face::FaceScope::NonLocal;
        let face_persistency = self.persistency;
        let mtu = Some(self.mtu);
        let link_type = face::LinkType::PointToPoint;

        face::FaceStatus {
            face_id,
            uri,
            local_uri,
            expiration_period,
            face_scope,
            face_persistency,
            mtu,
            link_type,
            base_congestion_marking_interval: self.base_congestion_marking_interval,
            default_congestion_threshold: self.default_congestion_threshold,
            n_in_interests: self.n_in_interests.into(),
            n_in_data: self.n_in_data.into(),
            n_in_nacks: self.n_in_nacks.into(),
            n_out_interests: self.n_out_interests.into(),
            n_out_data: self.n_out_data.into(),
            n_out_nacks: self.n_out_nacks.into(),
            n_in_bytes: self.n_in_bytes.into(),
            n_out_bytes: self.n_out_bytes.into(),
            flags: self.flags,
        }
    }
}
