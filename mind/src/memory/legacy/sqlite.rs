#![allow(dead_code)]
use crate::memory::contracts::{MemoryResult, MemoryStore};
use crate::memory::legacy::types::{Event, EventKind, Fact};
use rusqlite::{params, Connection};
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct SqliteMemoryStore {
    db_path: PathBuf,
}

impl SqliteMemoryStore {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        let db_path = path.into();
        if let Some(parent) = db_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        Self { db_path }
    }

    fn now_ts() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0)
    }

    fn open(&self) -> MemoryResult<Connection> {
        let conn = Connection::open(&self.db_path).map_err(|e| e.to_string())?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ws TEXT NOT NULL,
                trace TEXT NOT NULL,
                ts INTEGER NOT NULL,
                kind TEXT NOT NULL,
                payload TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS facts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                key TEXT NOT NULL,
                value TEXT NOT NULL,
                tags_json TEXT NOT NULL,
                ts INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ws TEXT NOT NULL,
                trace TEXT NOT NULL,
                created_ts INTEGER NOT NULL,
                last_ts INTEGER NOT NULL
            );",
        )
        .map_err(|e| e.to_string())?;
        Ok(conn)
    }

    pub fn path(&self) -> &Path {
        &self.db_path
    }
}

impl MemoryStore for SqliteMemoryStore {
    fn put_event(&self, ws: &str, trace: &str, kind: EventKind, payload: &str) -> MemoryResult<()> {
        if payload.len() > 64 * 1024 {
            return Err("payload exceeds 64KB".to_string());
        }
        let conn = self.open()?;
        let ts = Self::now_ts();
        conn.execute(
            "INSERT INTO events (ws, trace, ts, kind, payload) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![ws, trace, ts, kind.as_str(), payload],
        )
        .map_err(|e| e.to_string())?;

        conn.execute(
            "INSERT INTO sessions (ws, trace, created_ts, last_ts)
             VALUES (?1, ?2, ?3, ?3)
             ON CONFLICT(ws, trace) DO UPDATE SET last_ts = excluded.last_ts",
            params![ws, trace, ts],
        )
        .ok();

        Ok(())
    }

    fn recent_events(&self, ws: &str, limit: usize) -> MemoryResult<Vec<Event>> {
        let conn = self.open()?;
        let mut stmt = conn
            .prepare(
                "SELECT id, ws, trace, ts, kind, payload
                 FROM events
                 WHERE ws = ?1
                 ORDER BY ts DESC, id DESC
                 LIMIT ?2",
            )
            .map_err(|e| e.to_string())?;

        let mut rows = stmt
            .query(params![ws, limit as i64])
            .map_err(|e| e.to_string())?;
        let mut out = Vec::new();
        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let kind_str: String = row.get(4).map_err(|e| e.to_string())?;
            out.push(Event {
                id: row.get(0).map_err(|e| e.to_string())?,
                ws: row.get(1).map_err(|e| e.to_string())?,
                trace: row.get(2).map_err(|e| e.to_string())?,
                ts: row.get(3).map_err(|e| e.to_string())?,
                kind: EventKind::from_str(&kind_str),
                payload: row.get(5).map_err(|e| e.to_string())?,
            });
        }
        Ok(out)
    }

    fn put_fact(&self, key: &str, value: &str, tags: &[String]) -> MemoryResult<()> {
        if key.is_empty() {
            return Err("fact key empty".to_string());
        }
        let conn = self.open()?;
        let ts = Self::now_ts();
        let tags_json = json!(tags).to_string();
        conn.execute(
            "INSERT INTO facts (key, value, tags_json, ts) VALUES (?1, ?2, ?3, ?4)",
            params![key, value, tags_json, ts],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn search_facts(&self, query: &str, limit: usize) -> MemoryResult<Vec<Fact>> {
        let conn = self.open()?;
        let like = format!("%{}%", query);
        let mut stmt = conn
            .prepare(
                "SELECT id, key, value, tags_json, ts
                 FROM facts
                 WHERE key LIKE ?1 OR value LIKE ?1
                 ORDER BY ts DESC, id DESC
                 LIMIT ?2",
            )
            .map_err(|e| e.to_string())?;

        let mut rows = stmt
            .query(params![like, limit as i64])
            .map_err(|e| e.to_string())?;
        let mut out = Vec::new();
        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let tags_json: String = row.get(3).map_err(|e| e.to_string())?;
            let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
            out.push(Fact {
                id: row.get(0).map_err(|e| e.to_string())?,
                key: row.get(1).map_err(|e| e.to_string())?,
                value: row.get(2).map_err(|e| e.to_string())?,
                tags,
                ts: row.get(4).map_err(|e| e.to_string())?,
            });
        }
        Ok(out)
    }
}
