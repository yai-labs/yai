use crate::memory::graph::domains::episodic::types::Episode;
use crate::workspace::layout::WorkspaceLayout;
use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub struct EpisodicStore {
    path: PathBuf,
}

impl EpisodicStore {
    /// Dominio puro: riceve layout già risolto
    pub fn open(layout: &WorkspaceLayout) -> Result<Self> {
        let path = layout.events_log();
        Ok(Self { path })
    }

    pub fn ingest_events(&self) -> Result<Vec<Episode>> {
        if !self.path.exists() {
            return Ok(vec![]);
        }

        let f = File::open(&self.path).context("open events.log")?;
        let reader = BufReader::new(f);

        let allowed: HashSet<&'static str> = [
            "daemon_started",
            "daemon_stopped",
            "ws_up_started",
            "ws_up_complete",
            "ws_down_started",
            "ws_down_complete",
            "proc_started",
            "proc_exit",
            "kernel_dead",
            "status_changed",
            "provider_discovered",
            "provider_paired",
            "provider_attached",
            "provider_detached",
            "DATA_WRITE",
            "DATA_READ",
            "DATA_EXPORT",
            "DATA_ERASE",
            "PROCESSING_DECLARED",
            "RETENTION_EXPIRE",
            "error",
        ]
        .into_iter()
        .collect();

        let mut out = Vec::new();
        let mut seq: u64 = 0;

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            let v: Value = serde_json::from_str(&line).unwrap_or(Value::Null);

            let ts = v.get("ts").and_then(|v| v.as_u64()).unwrap_or(0);

            let raw_typ = v
                .get("type")
                .and_then(|v| v.as_str())
                .or_else(|| v.get("kind").and_then(|v| v.as_str()))
                .unwrap_or("untyped");

            let typ = if allowed.contains(raw_typ) {
                raw_typ
            } else {
                "untyped"
            }
            .to_string();

            let data = v.get("data").cloned().unwrap_or(Value::Null);
            let compliance = v.get("compliance").cloned();

            let id = format!("episode:{}:{}:{}", typ, ts, seq);

            out.push(Episode {
                id,
                ts,
                seq,
                event_type: typ,
                data,
                compliance,
            });

            seq += 1;
        }

        Ok(out)
    }
}
