use slotmap::Key;
use slotmap::KeyData;
use slotmap::SlotMap;
use tokio::sync::RwLock;
use tokio::sync::RwLockReadGuard;

use super::*;

use inner::Socket;

mod create;
mod inner;

#[derive(Debug)]
pub struct Face {
    face_id: face::FaceId,
    uri: face::Uri,
    local_uri: face::LocalUri,
    mtu: face::Mtu,
    persistency: face::FacePersistency,
    socket: Socket,
}

slotmap::new_key_type! { struct FaceKey; }

impl From<&face::FaceId> for FaceKey {
    fn from(face: &face::FaceId) -> Self {
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

    #[tracing::instrument]
    pub async fn send(&self, face: &face::FaceId, data: Bytes) -> io::Result<()> {
        self.get_face(face).await?.send(data).await
    }

    #[tracing::instrument]
    pub async fn recv(&self, face: &face::FaceId) -> io::Result<Bytes> {
        self.get_face(face).await?.recv().await
    }

    pub async fn get_faces(&self) -> Vec<face::FaceId> {
        self.faces
            .read()
            .await
            .values()
            .map(|face| face.face_id())
            .cloned()
            .collect()
    }

    pub async fn get_face(&self, face: &face::FaceId) -> io::Result<RwLockReadGuard<'_, Face>> {
        let key = face.into();
        let faces = self.faces.read().await;
        RwLockReadGuard::try_map(faces, |faces| faces.get(key))
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

    // async fn get_face_io(&self, face: &face::FaceId) -> io::Result<RwLockReadGuard<'_, Face>> {
    //     self.get_face_impl(face)
    //         .await
    //         .ok_or_else(|| io::Error::other("FaceId not found"))
    // }
}
