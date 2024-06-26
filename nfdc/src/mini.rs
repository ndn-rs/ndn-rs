use super::*;

#[derive(Debug)]
pub(crate) struct Router {
    // router: router::Router,
    faces: router::FaceManegement,
}

impl Router {
    pub(crate) async fn new() -> anyhow::Result<Self> {
        let faces = router::FaceManegement::new();
        // let router = router::Router::new();

        for uri in [
            "tcp4://localhost:6363",
            // "unix:///run/nfd.sock",
            // "tcp4://anchor.local:6363",
            // "tcp4://anchor.local:6363",
            // "tcp4://anchor.local:6363",
            // "tcp4://anchor.local:6363",
            // "tcp4://anchor.local:6363",
            // "tcp4://anchor.local:6363",
            // "tcp4://anchor.local:6363",
        ] {
            let params = Self::tcp_face(uri);
            let response = faces.create(params).await;
            anyhow::ensure!(response.status_code.is_ok(), response.status_text);
        }
        Ok(Self { faces })
    }

    fn tcp_face(uri: &str) -> mgmt::ControlParameters {
        mgmt::ControlParameters::create_face(uri).mtu(1492)
    }

    pub(crate) async fn get_default_face(&self) -> face::FaceId {
        self.faces
            .get_faces()
            .await
            .pop()
            .expect("Need at least one configured face")
    }

    pub(crate) async fn send_item(
        &self,
        face: face::FaceId,
        packet: impl tlv::Tlv,
    ) -> io::Result<()> {
        self.faces.send_item(face, packet).await
    }

    pub(crate) async fn recv_item(&self, face: face::FaceId) -> io::Result<tlv::Generic> {
        self.faces.recv_item(face).await
    }

    pub(crate) fn info(&self) {
        println!("{:#?}", self.faces);
    }
}
