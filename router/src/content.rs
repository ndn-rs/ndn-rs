use super::*;

#[derive(Debug, Default)]
pub struct ContentStore {
    store: RwLock<HashMap<Interest, Data>>,
}

impl ContentStore {
    pub async fn lookup(&self, interest: &Interest) -> Option<RwLockReadGuard<'_, Data>> {
        let store = self.store.read().await;
        RwLockReadGuard::try_map(store, |store| store.get(interest)).ok()
    }
}
