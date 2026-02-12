use crate::interface::config::RuntimeConfig;
use crate::interface::paths;
use crate::interface::tui::app::{AppState, DbScope};
use crate::interface::tui::datasource::DataSource;
use crate::shared::constants::DEFAULT_KNOWLEDGE_DB_PATH;
use anyhow::Result;
use rusqlite::Connection;
use serde_json::Value;
use std::env;
use std::path::PathBuf;

pub struct DbSource;

impl DataSource for DbSource {
    fn tick(&mut self, _cfg: &RuntimeConfig, state: &mut AppState) -> Result<()> {
        let db_path = match state.db.selected_db {
            DbScope::Global => env::var("YAI_KNOWLEDGE_DB")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from(DEFAULT_KNOWLEDGE_DB_PATH)),
            DbScope::Workspace => paths::run_dir().join(&state.ws).join("semantic.sqlite"),
        };
        if !db_path.exists() {
            state.db.tables.clear();
            state.db.counts.clear();
            state.db.preview.clear();
            return Ok(());
        }
        let conn = Connection::open(&db_path)?;
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name ASC")?;
        let tbl_iter = stmt.query_map([], |r| r.get::<_, String>(0))?;
        let mut tables = Vec::new();
        for t in tbl_iter {
            tables.push(t?);
        }
        state.db.tables = tables.clone();
        state.db.counts.clear();
        for t in &tables {
            let q = format!("SELECT COUNT(*) FROM \"{}\"", t.replace('"', "\"\""));
            let c: i64 = conn.query_row(&q, [], |r| r.get(0)).unwrap_or(0);
            state.db.counts.insert(t.clone(), c);
        }
        let selected = state
            .db
            .selected_table
            .clone()
            .or_else(|| tables.first().cloned());
        state.db.selected_table = selected.clone();
        state.db.preview.clear();
        if let Some(tbl) = selected {
            let q = format!("SELECT * FROM \"{}\" LIMIT 20", tbl.replace('"', "\"\""));
            let mut stmt = conn.prepare(&q)?;
            let names: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();
            let mut rows = stmt.query([])?;
            while let Some(row) = rows.next()? {
                let mut obj = serde_json::Map::new();
                for (i, n) in names.iter().enumerate() {
                    let as_text: Result<String, _> = row.get(i);
                    obj.insert(
                        n.clone(),
                        Value::String(as_text.unwrap_or_else(|_| "<binary>".to_string())),
                    );
                }
                state.db.preview.push(Value::Object(obj));
            }
        }
        Ok(())
    }
}
