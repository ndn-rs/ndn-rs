use super::*;

#[derive(Debug, Default)]
pub struct FaceManegement {
    faces: HashMap<face::FaceId, Inner>,
}

#[derive(Debug)]
struct Inner {
    _socket: net::SocketAddr,
}

impl FaceManegement {
    pub fn new() -> Self {
        Self::default()
    }

    #[tracing::instrument]
    pub async fn create(&self, params: face::ControlParameters) -> io::Result<()> {
        Ok(())
    }

    #[tracing::instrument]
    pub async fn send(&self, face: face::FaceId, data: Bytes) -> io::Result<()> {
        Ok(())
    }

    #[tracing::instrument]
    pub async fn recv(&self, face: face::FaceId) -> io::Result<Bytes> {
        Ok(Bytes::new())
    }
    pub fn get_faces(&self) -> Vec<face::FaceId> {
        self.faces.keys().copied().collect()
    }
}

// struct Create {
//     face_id: face::FaceId,
//     uri: face::Uri,
//     // local_uri: Option<LocalUri>,
//     persistency: face::FacePersistency,
//     // base_congestion_marking_interval: Option<BaseCongestionMarkingInterval>,
//     // default_congestion_threshold: Option<DefaultCongestionThreshold>,
//     // mtu: Option<Mtu>,
//     // flags_and_mask: Option<(Flags, Mask)>,
// }
