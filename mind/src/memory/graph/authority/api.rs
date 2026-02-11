use crate::memory::graph::authority::store::AuthorityStore;
use crate::memory::graph::authority::types::AuthorityPolicy;
use anyhow::Result;

pub fn load(ws: &str) -> Result<Vec<AuthorityPolicy>> {
    let store = AuthorityStore::open(ws)?;
    store.load()
}
