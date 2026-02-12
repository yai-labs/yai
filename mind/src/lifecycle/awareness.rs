#![allow(dead_code)]
use crate::core::scheduler::Scheduler;
use crate::interface::paths;
use crate::memory::graph::facade::{GraphFacade, GraphScope};
use crate::memory::MemoryCore;
use anyhow::Result;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct AwarenessConfig {
    pub tick_ms: u64,
    pub max_steps: Option<u64>,
}

impl Default for AwarenessConfig {
    fn default() -> Self {
        Self {
            tick_ms: 250,
            max_steps: Some(10),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AwarenessIntent {
    GraphStats,
    GraphWarmup,
    Noop,
}

impl AwarenessIntent {
    fn as_str(&self) -> &'static str {
        match self {
            AwarenessIntent::GraphStats => "graph_stats",
            AwarenessIntent::GraphWarmup => "graph_warmup",
            AwarenessIntent::Noop => "noop",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AwarenessTick {
    pub ws: String,
    pub step: u64,
    pub intent: AwarenessIntent,
}

pub fn append_awareness_log(ws: &str, line: &str) -> Result<()> {
    let run_ws_dir = paths::run_dir().join(ws);
    fs::create_dir_all(&run_ws_dir)?;
    let log_path = run_ws_dir.join("awareness.log");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;
    writeln!(file, "{line}")?;
    Ok(())
}

pub fn run_awareness_with_config(ws: &str, cfg: AwarenessConfig) -> Result<()> {
    let mut step: u64 = 0;
    loop {
        if let Some(max_steps) = cfg.max_steps {
            if step >= max_steps {
                break;
            }
        }

        let tick = AwarenessTick {
            ws: ws.to_string(),
            step,
            intent: AwarenessIntent::GraphStats,
        };
        let stats = GraphFacade::stats(GraphScope::Workspace(tick.ws.clone()))?;
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let line = format!(
            "{ts} {} {} {} {} {}",
            tick.step,
            tick.intent.as_str(),
            stats.nodes,
            stats.edges,
            stats.backend
        );
        append_awareness_log(&tick.ws, &line)?;

        step += 1;
        thread::sleep(Duration::from_millis(cfg.tick_ms));
    }
    Ok(())
}

pub fn awareness_log_path(ws: &str) -> PathBuf {
    paths::run_dir().join(ws).join("awareness.log")
}

pub fn start_awareness(core: MemoryCore, scheduler: Scheduler, ws: String, trace: String) {
    let _ = (core, scheduler, trace);
    let _ = run_awareness_with_config(&ws, AwarenessConfig::default());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn awareness_writes_log() -> Result<()> {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let ws = format!("awareness_test_{ts}");
        run_awareness_with_config(
            &ws,
            AwarenessConfig {
                tick_ms: 5,
                max_steps: Some(3),
            },
        )?;
        let p = awareness_log_path(&ws);
        let content = std::fs::read_to_string(&p)?;
        assert!(!content.trim().is_empty());
        assert!(content.lines().count() >= 1);
        Ok(())
    }
}
