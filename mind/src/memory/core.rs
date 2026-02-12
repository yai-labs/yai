use crate::memory::graph::semantic;
use crate::memory::graph::semantic::types::NodeRetention;
use crate::memory::types::{Event, EventKind, Fact};
use serde_json::json;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

pub type MemoryResult<T> = Result<T, String>;

static EVENT_SEQ: AtomicU64 = AtomicU64::new(1);
static FACT_SEQ: AtomicU64 = AtomicU64::new(1);

pub struct MemoryCore;

impl MemoryCore {
    pub fn new() -> Self {
        Self
    }

    pub fn put_event(&self, ws: &str, trace: &str, kind: EventKind, payload: &str) -> MemoryResult<()> {
        if payload.len() > 64 * 1024 {
            return Err("payload exceeds 64KB".to_string());
        }
        let ts = now_ts();
        let seq = EVENT_SEQ.fetch_add(1, Ordering::Relaxed);
        let node_id = format!("node:conversation:{}:{}:{}", ws, ts, seq);
        let meta = json!({
            "ws": ws,
            "trace": trace,
            "kind": kind.as_str(),
            "payload": payload,
            "ts": ts,
        });
        let retention = NodeRetention {
            created_ts: ts as u64,
            retention_policy_id: "conversation".to_string(),
            ttl_seconds: None,
            compliance: None,
        };
        semantic::api::add_node_with_retention(ws, &node_id, "conversation_event", &meta, &retention)
            .map_err(|e| e.to_string())
    }

    pub fn recent_events(&self, ws: &str, limit: usize) -> MemoryResult<Vec<Event>> {
        let mut events = semantic::api::list_nodes(ws)
            .map_err(|e| e.to_string())?
            .into_iter()
            .filter(|n| n.kind == "conversation_event")
            .map(|n| {
                let trace = n
                    .meta
                    .get("trace")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let kind = n
                    .meta
                    .get("kind")
                    .and_then(|v| v.as_str())
                    .map(EventKind::from_str)
                    .unwrap_or(EventKind::System);
                let payload = n
                    .meta
                    .get("payload")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                Event {
                    id: parse_node_tail_i64(&n.id).unwrap_or(0),
                    ws: ws.to_string(),
                    trace,
                    ts: n.created_ts as i64,
                    kind,
                    payload,
                }
            })
            .collect::<Vec<_>>();

        events.sort_by(|a, b| b.ts.cmp(&a.ts).then_with(|| b.id.cmp(&a.id)));
        events.truncate(limit);
        Ok(events)
    }

    pub fn put_fact(&self, ws: &str, key: &str, value: &str, tags: &[String]) -> MemoryResult<()> {
        if key.is_empty() {
            return Err("fact key empty".to_string());
        }
        let ts = now_ts();
        let seq = FACT_SEQ.fetch_add(1, Ordering::Relaxed);
        let node_id = format!("node:fact:{}:{}:{}", ws, ts, seq);
        let meta = json!({
            "key": key,
            "value": value,
            "tags": tags,
            "ts": ts,
        });
        let retention = NodeRetention {
            created_ts: ts as u64,
            retention_policy_id: "fact".to_string(),
            ttl_seconds: None,
            compliance: None,
        };
        semantic::api::add_node_with_retention(ws, &node_id, "fact", &meta, &retention).map_err(|e| e.to_string())
    }

    pub fn search_facts(&self, ws: &str, query: &str, limit: usize) -> MemoryResult<Vec<Fact>> {
        let query_lc = query.to_lowercase();
        let mut facts = semantic::api::list_nodes(ws)
            .map_err(|e| e.to_string())?
            .into_iter()
            .filter(|n| n.kind == "fact")
            .filter_map(|n| {
                let key = n.meta.get("key").and_then(|v| v.as_str())?.to_string();
                let value = n.meta.get("value").and_then(|v| v.as_str())?.to_string();
                if !query_lc.is_empty()
                    && !key.to_lowercase().contains(&query_lc)
                    && !value.to_lowercase().contains(&query_lc)
                {
                    return None;
                }
                let tags = n
                    .meta
                    .get("tags")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(ToString::to_string))
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                Some(Fact {
                    id: parse_node_tail_i64(&n.id).unwrap_or(0),
                    key,
                    value,
                    tags,
                    ts: n.created_ts as i64,
                })
            })
            .collect::<Vec<_>>();

        facts.sort_by(|a, b| b.ts.cmp(&a.ts).then_with(|| b.id.cmp(&a.id)));
        facts.truncate(limit);
        Ok(facts)
    }
}

fn now_ts() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

fn parse_node_tail_i64(id: &str) -> Option<i64> {
    id.rsplit(':').next()?.parse::<i64>().ok()
}
