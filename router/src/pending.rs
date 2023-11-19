use super::*;

#[derive(Debug, Default)]
pub struct PendingInterestTable {
    pit: RwLock<HashMap<Interest, HashSet<FaceRef>>>,
}

impl PendingInterestTable {
    pub async fn register(&self, interest: &Interest, downstream: FaceRef) {
        let interest = interest.clone();
        self.pit
            .write()
            .await
            .entry(interest)
            .or_default()
            .insert(downstream);
    }
}
