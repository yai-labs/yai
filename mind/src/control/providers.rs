use crate::interface::paths;
use crate::memory::graph::semantic;
use crate::rpc::protocol::{ProviderInfo, TrustState};
use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TrustStore {
    version: u32,
    updated_at: u64,
    providers: Vec<ProviderRecord>,
    integrity: StoreIntegrity,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct StoreIntegrity {
    file_hash: String,
    signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProviderRecord {
    #[serde(flatten)]
    info: ProviderInfo,
    #[serde(default)]
    audit: ProviderAudit,
    #[serde(default)]
    integrity: RecordIntegrity,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct ProviderAudit {
    #[serde(default)]
    last_event_id: Option<String>,
    #[serde(default)]
    history_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct RecordIntegrity {
    #[serde(default)]
    record_hash: String,
    #[serde(default)]
    signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProviderFileLegacy {
    providers: Vec<ProviderInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProviderInfoV1 {
    id: String,
    endpoint: String,
    model: String,
    trusted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProviderFileV1 {
    providers: Vec<ProviderInfoV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ActiveProviders {
    active: Option<ProviderInfo>,
}

#[derive(Debug, Clone)]
pub struct TrustTransition {
    pub provider: ProviderInfo,
    pub from_state: Option<TrustState>,
    pub to_state: TrustState,
    pub trust_snapshot_hash: String,
}

fn trust_file() -> PathBuf {
    paths::trust_dir().join("providers.json")
}

fn legacy_global_file() -> PathBuf {
    paths::home_dir().join(".yai").join("providers.json")
}

fn active_file(run_dir: &Path, ws: &str) -> PathBuf {
    run_dir.join(ws).join("providers.json")
}

fn active_file_legacy(run_dir: &Path, ws: &str) -> PathBuf {
    run_dir.join(ws).join("provider.json")
}

fn now_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn hash_str(input: &str) -> String {
    let mut h = DefaultHasher::new();
    input.hash(&mut h);
    format!("{:016x}", h.finish())
}

fn signature_for(hash: &str) -> String {
    let key = env::var("YAI_TRUST_KEY").unwrap_or_else(|_| "yai-trust-local-key".to_string());
    hash_str(&format!("{}:{}", key, hash))
}

fn provider_payload(info: &ProviderInfo) -> String {
    serde_json::to_string(info).unwrap_or_else(|_| format!("{}:{}:{}", info.id, info.endpoint, info.model))
}

fn parse_legacy_providers(data: &str) -> Result<Vec<ProviderInfo>> {
    if let Ok(v2) = serde_json::from_str::<ProviderFileLegacy>(data) {
        return Ok(v2.providers);
    }
    let v1: ProviderFileV1 = serde_json::from_str(data).context("parse providers.json v1")?;
    let now = now_epoch();
    Ok(v1
        .providers
        .into_iter()
        .map(|p| ProviderInfo {
            id: p.id,
            endpoint: p.endpoint,
            model: p.model,
            trust_state: if p.trusted { TrustState::Paired } else { TrustState::Discovered },
            fingerprint: None,
            capabilities: Vec::new(),
            last_seen: now,
            attached_ws: None,
        })
        .collect())
}

fn merge_providers(dst: &mut Vec<ProviderInfo>, src: Vec<ProviderInfo>) {
    for item in src {
        if let Some(existing) = dst.iter_mut().find(|p| p.id == item.id) {
            if item.last_seen > existing.last_seen {
                *existing = item;
            }
        } else {
            dst.push(item);
        }
    }
}

fn collect_legacy() -> Result<Vec<ProviderInfo>> {
    let mut out: Vec<ProviderInfo> = Vec::new();

    let global = legacy_global_file();
    if global.exists() {
        let data = fs::read_to_string(&global).with_context(|| format!("read legacy providers: {}", global.display()))?;
        if !data.trim().is_empty() {
            merge_providers(&mut out, parse_legacy_providers(&data)?);
        }
    }

    let run = paths::run_dir();
    if run.is_dir() {
        for entry in fs::read_dir(&run).with_context(|| format!("read run dir: {}", run.display()))? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }
            let p = entry.path().join("providers.json");
            if p.exists() {
                let data = fs::read_to_string(&p).with_context(|| format!("read legacy ws providers: {}", p.display()))?;
                if !data.trim().is_empty() {
                    merge_providers(&mut out, parse_legacy_providers(&data)?);
                }
            }
        }
    }

    Ok(out)
}

fn recalc_record(record: &mut ProviderRecord) {
    let payload = provider_payload(&record.info);
    record.integrity.record_hash = hash_str(&payload);
    record.integrity.signature = signature_for(&record.integrity.record_hash);
}

fn recalc_store(store: &mut TrustStore) {
    for record in &mut store.providers {
        recalc_record(record);
    }
    let snapshot = serde_json::to_string(&store.providers).unwrap_or_default();
    store.integrity.file_hash = hash_str(&snapshot);
    store.integrity.signature = signature_for(&store.integrity.file_hash);
    store.updated_at = now_epoch();
}

fn save_store(mut store: TrustStore) -> Result<TrustStore> {
    let path = trust_file();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create trust dir: {}", parent.display()))?;
    }
    recalc_store(&mut store);
    let data = serde_json::to_string_pretty(&store).context("serialize trust store")?;
    fs::write(&path, data).with_context(|| format!("write trust store: {}", path.display()))?;
    Ok(store)
}

fn load_store() -> Result<TrustStore> {
    let path = trust_file();
    if path.exists() {
        let data = fs::read_to_string(&path).with_context(|| format!("read trust store: {}", path.display()))?;
        if !data.trim().is_empty() {
            if let Ok(store) = serde_json::from_str::<TrustStore>(&data) {
                return Ok(store);
            }
            if let Ok(legacy) = parse_legacy_providers(&data) {
                let providers = legacy
                    .into_iter()
                    .map(|info| {
                        let mut r = ProviderRecord {
                            info,
                            audit: ProviderAudit::default(),
                            integrity: RecordIntegrity::default(),
                        };
                        recalc_record(&mut r);
                        r
                    })
                    .collect();
                return save_store(TrustStore {
                    version: 1,
                    updated_at: now_epoch(),
                    providers,
                    integrity: StoreIntegrity::default(),
                });
            }
        }
    }

    let migrated = collect_legacy()?;
    let providers = migrated
        .into_iter()
        .map(|info| {
            let mut r = ProviderRecord {
                info,
                audit: ProviderAudit::default(),
                integrity: RecordIntegrity::default(),
            };
            recalc_record(&mut r);
            r
        })
        .collect();
    save_store(TrustStore {
        version: 1,
        updated_at: now_epoch(),
        providers,
        integrity: StoreIntegrity::default(),
    })
}

fn to_transition(store: &TrustStore, old: Option<TrustState>, rec: &ProviderRecord) -> TrustTransition {
    TrustTransition {
        provider: rec.info.clone(),
        from_state: old,
        to_state: rec.info.trust_state.clone(),
        trust_snapshot_hash: store.integrity.file_hash.clone(),
    }
}

pub fn record_audit_event(provider_id: &str, event_id: &str) -> Result<()> {
    let mut store = load_store()?;
    if let Some(rec) = store.providers.iter_mut().find(|p| p.info.id == provider_id) {
        rec.audit.last_event_id = Some(event_id.to_string());
        let base = format!("{}:{}", rec.audit.history_hash, event_id);
        rec.audit.history_hash = hash_str(&base);
        recalc_record(rec);
        let _ = save_store(store)?;
    }
    Ok(())
}

pub fn discover(endpoint: Option<String>, model: Option<String>) -> Result<Vec<TrustTransition>> {
    let endpoint = endpoint
        .or_else(|| env::var("YAI_REMOTE_ENDPOINT").ok())
        .unwrap_or_default();
    if endpoint.trim().is_empty() {
        return Ok(Vec::new());
    }
    let model = model
        .or_else(|| env::var("YAI_REMOTE_MODEL").ok())
        .unwrap_or_else(|| "unknown".to_string());
    let id = format!("remote:{}", endpoint);
    let now = now_epoch();

    let mut store = load_store()?;
    let mut old_state = None;
    let idx = if let Some(i) = store.providers.iter().position(|p| p.info.id == id) {
        old_state = Some(store.providers[i].info.trust_state.clone());
        i
    } else {
        store.providers.push(ProviderRecord {
            info: ProviderInfo {
                id: id.clone(),
                endpoint: endpoint.clone(),
                model: model.clone(),
                trust_state: TrustState::Discovered,
                fingerprint: None,
                capabilities: Vec::new(),
                last_seen: now,
                attached_ws: None,
            },
            audit: ProviderAudit::default(),
            integrity: RecordIntegrity::default(),
        });
        store.providers.len() - 1
    };

    let rec = &mut store.providers[idx];
    rec.info.endpoint = endpoint;
    rec.info.model = model;
    rec.info.last_seen = now;
    if matches!(rec.info.trust_state, TrustState::Revoked) {
        bail!("provider is revoked");
    }

    let store = save_store(store)?;
    let rec = store
        .providers
        .iter()
        .find(|p| p.info.id == id)
        .ok_or_else(|| anyhow!("provider missing after save"))?;
    Ok(vec![to_transition(&store, old_state, rec)])
}

pub fn list_all() -> Result<Vec<ProviderInfo>> {
    let store = load_store()?;
    Ok(store.providers.into_iter().map(|p| p.info).collect())
}

pub fn list_trusted() -> Result<Vec<ProviderInfo>> {
    let store = load_store()?;
    Ok(store
        .providers
        .into_iter()
        .map(|p| p.info)
        .filter(|p| matches!(p.trust_state, TrustState::Paired | TrustState::Attached | TrustState::Detached))
        .collect())
}

pub fn pair(mut info: ProviderInfo) -> Result<TrustTransition> {
    let mut store = load_store()?;
    info.trust_state = TrustState::Paired;
    info.attached_ws = None;
    info.last_seen = now_epoch();

    let mut old_state = None;
    if let Some(rec) = store.providers.iter_mut().find(|p| p.info.id == info.id) {
        old_state = Some(rec.info.trust_state.clone());
        if matches!(rec.info.trust_state, TrustState::Revoked) {
            bail!("provider is revoked");
        }
        rec.info = info.clone();
    } else {
        store.providers.push(ProviderRecord {
            info: info.clone(),
            audit: ProviderAudit::default(),
            integrity: RecordIntegrity::default(),
        });
    }

    let store = save_store(store)?;
    let rec = store
        .providers
        .iter()
        .find(|p| p.info.id == info.id)
        .ok_or_else(|| anyhow!("provider missing after pair"))?;
    Ok(to_transition(&store, old_state, rec))
}

pub fn attach(run_dir: &Path, ws: &str, info: ProviderInfo) -> Result<TrustTransition> {
    let mut store = load_store()?;
    let rec = store
        .providers
        .iter_mut()
        .find(|p| p.info.id == info.id)
        .ok_or_else(|| anyhow!("provider not found (pair first)"))?;

    let old_state = Some(rec.info.trust_state.clone());
    if matches!(rec.info.trust_state, TrustState::Revoked | TrustState::Discovered) {
        bail!("provider not paired");
    }

    if let Some(attached_ws) = &rec.info.attached_ws {
        if attached_ws != ws {
            bail!("provider already attached to ws {}", attached_ws);
        }
    }

    rec.info.model = info.model;
    rec.info.trust_state = TrustState::Attached;
    rec.info.attached_ws = Some(ws.to_string());
    rec.info.last_seen = now_epoch();

    let store = save_store(store)?;
    let rec = store
        .providers
        .iter()
        .find(|p| p.info.id == info.id)
        .ok_or_else(|| anyhow!("provider missing after attach"))?;

    let path = active_file(run_dir, ws);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create ws dir: {}", parent.display()))?;
    }
    let data = serde_json::to_string_pretty(&ActiveProviders {
        active: Some(rec.info.clone()),
    })
    .context("serialize active provider")?;
    fs::write(&path, data).with_context(|| format!("write active provider: {}", path.display()))?;

    sync_graph(ws, &rec.info)?;
    Ok(to_transition(&store, old_state, rec))
}

pub fn detach(run_dir: &Path, ws: &str) -> Result<Option<TrustTransition>> {
    let path = active_file(run_dir, ws);
    if path.exists() {
        fs::remove_file(&path)
            .with_context(|| format!("remove active providers: {}", path.display()))?;
    }
    let legacy = active_file_legacy(run_dir, ws);
    if legacy.exists() {
        let _ = fs::remove_file(&legacy);
    }

    let mut store = load_store()?;
    if let Some(rec) = store
        .providers
        .iter_mut()
        .find(|p| p.info.attached_ws.as_deref() == Some(ws))
    {
        let old_state = Some(rec.info.trust_state.clone());
        if matches!(rec.info.trust_state, TrustState::Revoked) {
            bail!("provider is revoked");
        }
        rec.info.trust_state = TrustState::Detached;
        rec.info.attached_ws = None;
        rec.info.last_seen = now_epoch();
        let id = rec.info.id.clone();

        let store = save_store(store)?;
        let rec = store
            .providers
            .iter()
            .find(|p| p.info.id == id)
            .ok_or_else(|| anyhow!("provider missing after detach"))?;
        sync_graph(ws, &rec.info)?;
        return Ok(Some(to_transition(&store, old_state, rec)));
    }

    Ok(None)
}

pub fn status(run_dir: &Path, ws: &str) -> Result<Option<ProviderInfo>> {
    let path = active_file(run_dir, ws);
    if path.exists() {
        let data = fs::read_to_string(&path)
            .with_context(|| format!("read active providers: {}", path.display()))?;
        let file: ActiveProviders = serde_json::from_str(&data).context("parse active providers")?;
        return Ok(file.active);
    }

    let legacy = active_file_legacy(run_dir, ws);
    if legacy.exists() {
        let data = fs::read_to_string(&legacy)
            .with_context(|| format!("read legacy active provider: {}", legacy.display()))?;
        let file: ActiveProviders = serde_json::from_str(&data).context("parse legacy active provider")?;
        return Ok(file.active);
    }

    Ok(None)
}

pub fn revoke(id: &str) -> Result<Option<TrustTransition>> {
    let mut store = load_store()?;
    if let Some(rec) = store.providers.iter_mut().find(|p| p.info.id == id) {
        let old_state = Some(rec.info.trust_state.clone());
        rec.info.trust_state = TrustState::Revoked;
        rec.info.attached_ws = None;
        rec.info.last_seen = now_epoch();

        let store = save_store(store)?;
        let rec = store
            .providers
            .iter()
            .find(|p| p.info.id == id)
            .ok_or_else(|| anyhow!("provider missing after revoke"))?;
        return Ok(Some(to_transition(&store, old_state, rec)));
    }
    Ok(None)
}

pub fn get(id: &str) -> Result<Option<ProviderInfo>> {
    let store = load_store()?;
    Ok(store
        .providers
        .into_iter()
        .map(|p| p.info)
        .find(|p| p.id == id))
}

fn slugify(value: &str) -> String {
    value
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

fn provider_node_id(id: &str) -> String {
    format!("node:provider:{}", slugify(id))
}

fn capability_node_id(cap: &str) -> String {
    format!("node:capability:{}", slugify(cap))
}

fn workspace_node_id(ws: &str) -> String {
    format!("node:workspace:{}", ws)
}

fn kernel_node_id(ws: &str) -> String {
    format!("node:kernel:{}", ws)
}

pub fn sync_graph(ws: &str, info: &ProviderInfo) -> Result<()> {
    let provider_id = provider_node_id(&info.id);
    let meta = json!({
        "provider_id": info.id,
        "endpoint": info.endpoint,
        "model": info.model,
        "trust_state": format!("{:?}", info.trust_state).to_lowercase(),
        "last_seen": info.last_seen,
        "attached_ws": info.attached_ws,
    });
    semantic::api::add_node(ws, &provider_id, "provider", &meta)?;

    if matches!(
        info.trust_state,
        TrustState::Paired | TrustState::Attached | TrustState::Detached
    ) {
        let kernel_id = kernel_node_id(ws);
        semantic::api::add_node(ws, &kernel_id, "kernel", &json!({"ws": ws}))?;
        let edge_id = format!("edge:trusted_by:{}:{}", provider_id, kernel_id);
        semantic::api::add_edge(ws, &edge_id, &provider_id, &kernel_id, "trusted_by", 1.0)?;
    }

    if matches!(info.trust_state, TrustState::Attached) {
        let ws_node = workspace_node_id(ws);
        semantic::api::add_node(ws, &ws_node, "workspace", &json!({"ws": ws}))?;
        let edge_id = format!("edge:attached_to:{}:{}", provider_id, ws_node);
        semantic::api::add_edge(ws, &edge_id, &provider_id, &ws_node, "attached_to", 1.0)?;
    }

    for cap in &info.capabilities {
        let cap_id = capability_node_id(cap);
        semantic::api::add_node(ws, &cap_id, "capability", &json!({"cap": cap}))?;
        let edge_id = format!("edge:has_capability:{}:{}", provider_id, cap_id);
        semantic::api::add_edge(ws, &edge_id, &provider_id, &cap_id, "has_capability", 1.0)?;
    }
    Ok(())
}

pub fn trust_snapshot_hash() -> Result<String> {
    Ok(load_store()?.integrity.file_hash)
}

pub fn trust_snapshot() -> Result<Value> {
    let store = load_store()?;
    serde_json::to_value(store).context("serialize trust snapshot")
}
