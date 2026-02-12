use crate::cli::paths;
use crate::cognition::memory::graph::semantic::types::{
    ExpiredSemanticNode, NodeRetention, SemanticEdge, SemanticNode,
};
use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct SemanticStore {
    db_path: PathBuf,
}

impl SemanticStore {
    pub fn open(ws: &str) -> Result<Self> {
        let db_path = paths::run_dir().join(ws).join("semantic.sqlite");
        Self::open_at_path(db_path)
    }

    pub fn open_at_path(db_path: PathBuf) -> Result<Self> {
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent).context("create semantic db parent dir")?;
        }
        let store = Self { db_path };
        store.init()?;
        Ok(store)
    }

    pub fn path(&self) -> &Path {
        &self.db_path
    }

    fn init(&self) -> Result<()> {
        let conn = self.conn()?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS nodes (
                id TEXT PRIMARY KEY,
                kind TEXT NOT NULL,
                meta TEXT NOT NULL,
                last_seen INTEGER NOT NULL,
                created_ts INTEGER NOT NULL DEFAULT 0,
                expires_at INTEGER,
                retention_policy_id TEXT NOT NULL DEFAULT 'default',
                tombstone INTEGER NOT NULL DEFAULT 0,
                compliance TEXT
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

        // Legacy db migration: older tables may miss retention columns.
        add_column_if_missing(&conn, "nodes", "created_ts", "INTEGER NOT NULL DEFAULT 0")?;
        add_column_if_missing(&conn, "nodes", "expires_at", "INTEGER")?;
        add_column_if_missing(
            &conn,
            "nodes",
            "retention_policy_id",
            "TEXT NOT NULL DEFAULT 'default'",
        )?;
        add_column_if_missing(&conn, "nodes", "tombstone", "INTEGER NOT NULL DEFAULT 0")?;
        add_column_if_missing(&conn, "nodes", "compliance", "TEXT")?;

        Ok(())
    }

    fn conn(&self) -> Result<Connection> {
        Connection::open(&self.db_path).context("open semantic db")
    }

    pub fn upsert_node(&self, id: &str, kind: &str, meta: &Value) -> Result<()> {
        self.upsert_node_with_retention(id, kind, meta, &NodeRetention::default())
    }

    pub fn upsert_node_with_retention(
        &self,
        id: &str,
        kind: &str,
        meta: &Value,
        retention: &NodeRetention,
    ) -> Result<()> {
        let conn = self.conn()?;
        let created_ts = retention.created_ts;
        let expires_at = retention
            .ttl_seconds
            .map(|ttl| created_ts.saturating_add(ttl));
        let meta_raw = serde_json::to_string(meta).unwrap_or_else(|_| "{}".to_string());
        let compliance_raw = retention
            .compliance
            .as_ref()
            .and_then(|v| serde_json::to_string(v).ok());
        conn.execute(
            "INSERT INTO nodes (id, kind, meta, last_seen, created_ts, expires_at, retention_policy_id, tombstone, compliance)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0, ?8)
             ON CONFLICT(id) DO UPDATE SET
               kind=excluded.kind,
               meta=excluded.meta,
               last_seen=excluded.last_seen,
               created_ts=excluded.created_ts,
               expires_at=excluded.expires_at,
               retention_policy_id=excluded.retention_policy_id,
               tombstone=0,
               compliance=excluded.compliance",
            params![
                id,
                kind,
                meta_raw,
                created_ts,
                created_ts,
                expires_at,
                retention.retention_policy_id,
                compliance_raw,
            ],
        )
        .context("upsert node")?;
        Ok(())
    }

    pub fn upsert_edge(
        &self,
        id: &str,
        src: &str,
        dst: &str,
        rel: &str,
        weight: f32,
    ) -> Result<()> {
        let conn = self.conn()?;
        conn.execute(
            "INSERT INTO edges (id, src, dst, rel, weight) VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(id) DO UPDATE SET src=excluded.src, dst=excluded.dst, rel=excluded.rel, weight=excluded.weight",
            params![id, src, dst, rel, weight],
        )
        .context("upsert edge")?;
        Ok(())
    }

    pub fn expire_due(&self, now_ts: u64) -> Result<Vec<ExpiredSemanticNode>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, retention_policy_id, expires_at, compliance
             FROM nodes
             WHERE tombstone = 0
               AND compliance IS NOT NULL
               AND expires_at IS NOT NULL
               AND expires_at <= ?1
             ORDER BY id ASC",
        )?;
        let mut rows = stmt.query(params![now_ts])?;
        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            let compliance_raw: Option<String> = row.get(3)?;
            let compliance = compliance_raw
                .as_deref()
                .and_then(|v| serde_json::from_str::<Value>(v).ok());
            out.push(ExpiredSemanticNode {
                id: row.get(0)?,
                retention_policy_id: row.get(1)?,
                expired_at: row.get(2)?,
                compliance,
            });
        }
        if !out.is_empty() {
            conn.execute(
                "UPDATE nodes
                 SET tombstone = 1
                 WHERE tombstone = 0
                   AND compliance IS NOT NULL
                   AND expires_at IS NOT NULL
                   AND expires_at <= ?1",
                params![now_ts],
            )?;
        }
        Ok(out)
    }

    pub fn list_nodes(&self) -> Result<Vec<SemanticNode>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, kind, meta, last_seen, created_ts, expires_at, retention_policy_id, tombstone, compliance
             FROM nodes
             WHERE tombstone = 0
             ORDER BY id ASC",
        )?;
        let mut rows = stmt.query([])?;
        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            let meta_raw: String = row.get(2)?;
            let meta = serde_json::from_str(&meta_raw).unwrap_or(Value::Null);
            let compliance_raw: Option<String> = row.get(8)?;
            let compliance = compliance_raw
                .as_deref()
                .and_then(|v| serde_json::from_str::<Value>(v).ok());
            out.push(SemanticNode {
                id: row.get(0)?,
                kind: row.get(1)?,
                meta,
                last_seen: row.get(3)?,
                created_ts: row.get(4)?,
                expires_at: row.get(5)?,
                retention_policy_id: row.get(6)?,
                tombstone: row.get::<_, i64>(7)? != 0,
                compliance,
            });
        }
        Ok(out)
    }

    pub fn get_node(&self, id: &str) -> Result<Option<SemanticNode>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, kind, meta, last_seen, created_ts, expires_at, retention_policy_id, tombstone, compliance
             FROM nodes
             WHERE tombstone = 0 AND id = ?1
             LIMIT 1",
        )?;
        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            let meta_raw: String = row.get(2)?;
            let meta = serde_json::from_str(&meta_raw).unwrap_or(Value::Null);
            let compliance_raw: Option<String> = row.get(8)?;
            let compliance = compliance_raw
                .as_deref()
                .and_then(|v| serde_json::from_str::<Value>(v).ok());
            return Ok(Some(SemanticNode {
                id: row.get(0)?,
                kind: row.get(1)?,
                meta,
                last_seen: row.get(3)?,
                created_ts: row.get(4)?,
                expires_at: row.get(5)?,
                retention_policy_id: row.get(6)?,
                tombstone: row.get::<_, i64>(7)? != 0,
                compliance,
            }));
        }
        Ok(None)
    }

    pub fn list_edges(&self) -> Result<Vec<SemanticEdge>> {
        let conn = self.conn()?;
        let mut stmt =
            conn.prepare("SELECT id, src, dst, rel, weight FROM edges ORDER BY id ASC")?;
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

    pub fn get_edges_for_node(&self, id: &str) -> Result<Vec<SemanticEdge>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, src, dst, rel, weight
             FROM edges
             WHERE src = ?1 OR dst = ?1
             ORDER BY id ASC",
        )?;
        let mut rows = stmt.query(params![id])?;
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

fn add_column_if_missing(conn: &Connection, table: &str, column: &str, ddl: &str) -> Result<()> {
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table))?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let name: String = row.get(1)?;
        if name == column {
            return Ok(());
        }
    }
    conn.execute(
        &format!("ALTER TABLE {} ADD COLUMN {} {}", table, column, ddl),
        [],
    )
    .with_context(|| format!("add {}.{}", table, column))?;
    Ok(())
}
