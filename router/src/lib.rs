use std::collections::{HashMap, HashSet};
use std::io;

use tokio::sync::RwLock;
use tokio::sync::RwLockMappedWriteGuard;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;

use ndn_face as face;
use ndn_management as mgmt;
use ndn_tlv as tlv;
use ndn_transport as transport;

use tlv::Data;
use tlv::Interest;
use tlv::Tlv;
// use tlv::TlvCodec;

pub use content::ContentStore;
pub use error::Error;
pub use faces::Face;
pub use faces::FaceManegement;
pub use forwarding::ForwardingInformationBase;
pub use pending::PendingInterestTable;

mod content;
mod error;
mod faces;
mod forwarding;
mod pending;

#[derive(Debug, Default)]
pub struct Router {
    faces: FaceManegement,
    pending_interest_table: PendingInterestTable,
    forwarding_information_base: ForwardingInformationBase,
    content_store: ContentStore,
}

impl Router {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn with_internal_face() -> io::Result<Self> {
        let router = Self::new();
        let response = router.add_internal_face().await.into_result()?;
        tracing::debug!(?response, "add_internal_face");
        Ok(router)
    }

    pub async fn add_internal_face(&self) -> mgmt::ControlResponse {
        let params = mgmt::ControlParameters::create_face("internal://");
        self.handle_create_face(params).await
    }

    pub async fn handle_create_face(
        &self,
        params: mgmt::ControlParameters,
    ) -> mgmt::ControlResponse {
        self.faces.create(params).await
    }

    pub async fn handle_face_status(&self, face: face::FaceId) -> io::Result<face::FaceStatus> {
        self.faces
            .get_face(face)
            .await
            .map(|face| face.to_face_status())
    }

    pub async fn handle_interest(
        &self,
        interest: Interest,
        downstream: face::FaceId,
    ) -> io::Result<()> {
        if let Some(data) = self.content_store.lookup(&interest).await {
            // TODO Check freshness
            let data = data.clone();
            self.faces.send_item(downstream, data).await?;
        } else {
            self.pending_interest_table
                .register(&interest, downstream)
                .await;
            let upstream = self.forwarding_information_base.lookup(&interest).await;
            self.faces.send_item(upstream, interest).await?;
        }

        Ok(())
    }
}
