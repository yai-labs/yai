use crate::transport::rpc::protocol::{DsarRecord, DsarStatus};
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

fn dsar_path(run_dir: &Path, ws: &str) -> PathBuf {
    run_dir.join(ws).join("dsar.json")
}

fn load_all(run_dir: &Path, ws: &str) -> Result<Vec<DsarRecord>> {
    let path = dsar_path(run_dir, ws);
    if !path.exists() {
        return Ok(Vec::new());
    }
    let raw = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    let records = serde_json::from_str::<Vec<DsarRecord>>(&raw)
        .with_context(|| format!("parse {}", path.display()))?;
    Ok(records)
}

fn save_all(run_dir: &Path, ws: &str, records: &[DsarRecord]) -> Result<()> {
    let path = dsar_path(run_dir, ws);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("mkdir {}", parent.display()))?;
    }
    fs::write(&path, serde_json::to_string_pretty(records)?)
        .with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

pub fn create_request(
    run_dir: &Path,
    ws: &str,
    request_type: &str,
    subject_ref: &str,
) -> Result<DsarRecord> {
    if request_type != "export" && request_type != "erase" {
        anyhow::bail!("invalid dsar request_type: {}", request_type);
    }
    if subject_ref.trim().is_empty() {
        anyhow::bail!("invalid dsar subject_ref");
    }

    let mut records = load_all(run_dir, ws)?;
    let request_id = format!("dsar:{}:{}", ws, records.len() + 1);
    let rec = DsarRecord {
        request_id,
        subject_ref: subject_ref.to_string(),
        request_type: request_type.to_string(),
        status: DsarStatus::Requested,
    };
    records.push(rec.clone());
    save_all(run_dir, ws, &records)?;
    Ok(rec)
}

pub fn get_request(run_dir: &Path, ws: &str, request_id: &str) -> Result<Option<DsarRecord>> {
    let records = load_all(run_dir, ws)?;
    Ok(records.into_iter().find(|r| r.request_id == request_id))
}

pub fn set_status(
    run_dir: &Path,
    ws: &str,
    request_id: &str,
    status: DsarStatus,
) -> Result<Option<DsarRecord>> {
    let mut records = load_all(run_dir, ws)?;
    let mut found: Option<DsarRecord> = None;
    for r in &mut records {
        if r.request_id == request_id {
            r.status = status.clone();
            found = Some(r.clone());
            break;
        }
    }
    if found.is_some() {
        save_all(run_dir, ws, &records)?;
    }
    Ok(found)
}
