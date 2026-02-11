use crate::rpc::protocol::Event;
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

    pub fn emit(&self, kind: &str, data: Value) -> Event {
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
            event_id: format!("evt:{}:{}", ws, seq),
            ts,
            ws,
            kind: kind.to_string(),
            level: "info".to_string(),
            msg: kind.to_string(),
            seq,
            data,
        };
        self.append_log(&event);
        let _ = self.sender.send(event.clone());
        event
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
