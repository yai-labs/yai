use crate::interface::config::RuntimeConfig;
use crate::interface::paths;
use crate::interface::tui::app::{AppState, ProviderItem};
use crate::interface::tui::datasource::DataSource;
use anyhow::Result;
use serde_json::Value;
use std::fs;

pub struct ProvidersSource;

impl DataSource for ProvidersSource {
    fn tick(&mut self, _cfg: &RuntimeConfig, state: &mut AppState) -> Result<()> {
        let p = paths::trust_dir().join("providers.json");
        if !p.exists() {
            state.providers.list.clear();
            return Ok(());
        }
        let raw = fs::read_to_string(&p)?;
        let v: Value = serde_json::from_str(&raw)?;
        let items = v
            .get("providers")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let mut out = Vec::new();
        for it in items {
            out.push(ProviderItem {
                id: it.get("id").and_then(Value::as_str).unwrap_or("").to_string(),
                endpoint: it
                    .get("endpoint")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
                model: it
                    .get("model")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
                trust_state: it
                    .get("trust_state")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
                last_seen: it.get("last_seen").and_then(Value::as_i64).unwrap_or(0),
            });
        }
        out.sort_by(|a, b| {
            trust_rank(&b.trust_state)
                .cmp(&trust_rank(&a.trust_state))
                .then_with(|| b.last_seen.cmp(&a.last_seen))
        });
        state.providers.list = out;
        let preferred = state
            .providers
            .list
            .iter()
            .find(|p| !matches!(p.trust_state.as_str(), "revoked"))
            .or_else(|| state.providers.list.first())
            .map(|p| p.id.clone());

        if let Some(current) = state.providers.selected.clone() {
            let current_is_bad = state
                .providers
                .list
                .iter()
                .find(|p| p.id == current)
                .map(|p| matches!(p.trust_state.as_str(), "revoked"))
                .unwrap_or(true);
            if current_is_bad {
                state.providers.selected = preferred.clone();
            }
        } else {
            state.providers.selected = preferred.clone();
        }
        if let Some(sel) = &state.providers.selected {
            if let Some(p) = state.providers.list.iter().find(|p| &p.id == sel) {
                state.providers.trust = p.trust_state.clone();
                state.chat.provider_session = Some(format!("{} ({})", p.id, p.model));
            }
        }
        Ok(())
    }
}

fn trust_rank(state: &str) -> u8 {
    match state {
        "attached" => 5,
        "paired" => 4,
        "detached" => 3,
        "discovered" => 2,
        "revoked" => 1,
        _ => 0,
    }
}
