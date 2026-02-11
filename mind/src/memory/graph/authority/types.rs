use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorityPolicy {
    pub id: String,
    pub source: String,
}
