use crate::transport::rpc::protocol::{ComplianceContext, Event};
use anyhow::{anyhow, Result};
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<Event>,
    ws: String,
    run_dir: PathBuf,
    seq: Arc<AtomicU64>,
}

impl EventBus {
    pub fn new(run_dir: PathBuf, ws: String) -> Self {
        let (sender, _) = broadcast::channel(128);
        Self {
            sender,
            ws,
            run_dir,
            seq: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }

    pub fn emit(&self, kind: &str, data: Value) -> Result<Event> {
        self.emit_with_compliance(kind, data, None)
    }

    pub fn emit_with_compliance(
        &self,
        kind: &str,
        data: Value,
        compliance: Option<ComplianceContext>,
    ) -> Result<Event> {
        if kind.trim().is_empty() || kind.chars().any(|c| c.is_whitespace()) {
            return Err(anyhow!("invalid event kind"));
        }
        Self::validate_compliance(kind, compliance.as_ref())?;

        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let seq = self.seq.fetch_add(1, Ordering::Relaxed);
        let ws = data
            .get("ws")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| self.ws.clone());
        let event = Event {
            v: 1,
            schema_id: Some("mind.event.v1".to_string()),
            event_id: format!("evt:{}:{}", ws, seq),
            ts,
            ws,
            kind: kind.to_string(),
            level: "info".to_string(),
            msg: kind.to_string(),
            seq,
            data,
            compliance,
        };
        self.append_log(&event);
        let _ = self.sender.send(event.clone());
        Ok(event)
    }

    fn validate_compliance(kind: &str, compliance: Option<&ComplianceContext>) -> Result<()> {
        match kind {
            "DATA_WRITE" | "DATA_READ" | "DATA_EXPORT" | "DATA_ERASE" | "PROCESSING_DECLARED" => {
                if compliance.is_none() {
                    return Err(anyhow!("Missing compliance_context"));
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn append_log(&self, event: &Event) {
        let path = self.run_dir.join(&self.ws).join("events.log");
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&path) {
            if let Ok(line) = serde_json::to_string(event) {
                let _ = writeln!(f, "{}", line);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::rpc::protocol::ComplianceContext;
    use serde_json::json;

    #[test]
    fn data_event_requires_compliance_context() {
        let run_dir = std::env::temp_dir().join(format!("yai_events_test_{}", std::process::id()));
        let bus = EventBus::new(run_dir, "tws".to_string());
        let err = bus.emit("DATA_WRITE", json!({"ws": "tws"}));
        assert!(err.is_err());
    }

    #[test]
    fn data_event_with_compliance_context_is_ok() {
        let run_dir =
            std::env::temp_dir().join(format!("yai_events_test_{}_2", std::process::id()));
        let bus = EventBus::new(run_dir, "tws".to_string());
        let ctx = ComplianceContext {
            pack_ref: "gdpr-eu/2026Q1".to_string(),
            purpose_id: "LEGAL_OBLIGATION".to_string(),
            data_class: "PERSONAL".to_string(),
            retention_policy_id: "default".to_string(),
            legal_basis: "LEGAL_OBLIGATION".to_string(),
            subject_scope: "identified".to_string(),
            processor_role: "controller".to_string(),
            audit_required: true,
        };
        let ok = bus.emit_with_compliance("DATA_WRITE", json!({"ws": "tws"}), Some(ctx));
        assert!(ok.is_ok());
    }
}
