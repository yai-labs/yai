use crate::interface::paths;
use crate::memory::graph::semantic::types::{SemanticEdge, SemanticNode};
use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct SemanticStore {
    db_path: PathBuf,
}

impl SemanticStore {
    pub fn open(ws: &str) -> Result<Self> {
        let base = paths::run_dir().join(ws);
        fs::create_dir_all(&base).context("create ws dir")?;
        let db_path = base.join("semantic.sqlite");
        let store = Self { db_path };
        store.init()?;
        Ok(store)
    }

    fn init(&self) -> Result<()> {
        let conn = self.conn()?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS nodes (
                id TEXT PRIMARY KEY,
                kind TEXT NOT NULL,
                meta TEXT NOT NULL,
                last_seen INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS edges (
                id TEXT PRIMARY KEY,
                src TEXT NOT NULL,
                dst TEXT NOT NULL,
                rel TEXT NOT NULL,
                weight REAL NOT NULL
            );",
        )
        .context("init semantic db")?;
        Ok(())
    }

    fn conn(&self) -> Result<Connection> {
        Connection::open(&self.db_path).context("open semantic db")
    }

    pub fn upsert_node(&self, id: &str, kind: &str, meta: &Value) -> Result<()> {
        let conn = self.conn()?;
        let ts = now_epoch();
        let meta_raw = serde_json::to_string(meta).unwrap_or_else(|_| "{}".to_string());
        conn.execute(
            "INSERT INTO nodes (id, kind, meta, last_seen) VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(id) DO UPDATE SET kind=excluded.kind, meta=excluded.meta, last_seen=excluded.last_seen",
            params![id, kind, meta_raw, ts],
        )
        .context("upsert node")?;
        Ok(())
    }

    pub fn upsert_edge(&self, id: &str, src: &str, dst: &str, rel: &str, weight: f32) -> Result<()> {
        let conn = self.conn()?;
        conn.execute(
            "INSERT INTO edges (id, src, dst, rel, weight) VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(id) DO UPDATE SET src=excluded.src, dst=excluded.dst, rel=excluded.rel, weight=excluded.weight",
            params![id, src, dst, rel, weight],
        )
        .context("upsert edge")?;
        Ok(())
    }

    pub fn list_nodes(&self) -> Result<Vec<SemanticNode>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare("SELECT id, kind, meta, last_seen FROM nodes ORDER BY id ASC")?;
        let mut rows = stmt.query([])?;
        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            let meta_raw: String = row.get(2)?;
            let meta = serde_json::from_str(&meta_raw).unwrap_or(Value::Null);
            out.push(SemanticNode {
                id: row.get(0)?,
                kind: row.get(1)?,
                meta,
                last_seen: row.get(3)?,
            });
        }
        Ok(out)
    }

    pub fn list_edges(&self) -> Result<Vec<SemanticEdge>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare("SELECT id, src, dst, rel, weight FROM edges ORDER BY id ASC")?;
        let mut rows = stmt.query([])?;
        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            out.push(SemanticEdge {
                id: row.get(0)?,
                src: row.get(1)?,
                dst: row.get(2)?,
                rel: row.get(3)?,
                weight: row.get(4)?,
            });
        }
        Ok(out)
    }
}

fn now_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
