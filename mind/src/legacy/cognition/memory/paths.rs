use std::path::PathBuf;

use crate::paths::Paths;

/// Legacy helpers for filesystem locations (workspace-aware now).
pub fn semantic_sqlite(paths: &Paths) -> PathBuf {
    paths.semantic_sqlite()
}

pub fn events_log(paths: &Paths) -> PathBuf {
    paths.events_log()
}

pub fn knowledge_db_default(paths: &Paths) -> PathBuf {
    // If you still keep global knowledge db under repo/Data/db/knowledge.db,
    // you can wire that later. For now: place it under ~/.yai/db/knowledge.db
    paths.root.join("db").join("knowledge.db")
}
