use std::collections::{HashMap, HashSet};

use face::NdnResult;
use tokio::sync::{RwLock, RwLockReadGuard};

use ndn_face as face;
use ndn_tlv as tlv;

use face::FaceRef;
use tlv::Interest;
use tlv::{Data, Tlv};

pub use content::ContentStore;
pub use error::Error;
pub use forwarding::ForwardingInformationBase;
pub use pending::PendingInterestTable;

mod content;
mod error;
mod forwarding;
mod pending;

#[derive(Debug, Default)]
pub struct Router {
    pending_interest_table: PendingInterestTable,
    forwarding_information_base: ForwardingInformationBase,
    content_store: ContentStore,
}

impl Router {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn handle_interest(&self, interest: Interest, downstream: FaceRef) -> NdnResult<()> {
        if let Some(data) = self.content_store.lookup(&interest).await {
            // TODO Check freshness
            let bytes = data.bytes();
            downstream.send(bytes).await?;
        } else {
            self.pending_interest_table
                .register(&interest, downstream)
                .await;
            let upstream = self.forwarding_information_base.lookup(&interest).await;
            let bytes = interest.bytes();
            upstream.send(bytes).await?;
        }

        Ok(())
    }
}
