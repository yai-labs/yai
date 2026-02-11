use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::{Pid, ProcessStatus, System};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RunState {
    pub ws: String,
    pub boot_pid: Option<u32>,
    pub kernel_pid: Option<u32>,
    pub engine_pid: Option<u32>,
    pub mind_pid: Option<u32>,
    #[serde(default)]
    pub pgid: Option<u32>,
    pub socket_path: String,
    pub artifacts_root: String,
    pub started_at_epoch: u64,
}

pub fn pidfile_path(run_dir: &Path, ws: &str) -> PathBuf {
    run_dir.join(ws).join("session.json")
}

pub fn write_run_state(path: &Path, state: &RunState) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create run dir: {}", parent.display()))?;
    }
    let data = serde_json::to_string_pretty(state).context("serialize run state")?;
    fs::write(path, data).with_context(|| format!("write pidfile: {}", path.display()))?;
    Ok(())
}

pub fn read_run_state(path: &Path) -> Result<RunState> {
    let mut target = path.to_path_buf();
    if !target.exists() {
        if let Some(ws) = path.parent().and_then(|p| p.file_name()).and_then(|s| s.to_str()) {
            let legacy = path.parent().and_then(|p| p.parent()).map(|p| p.join(format!("{}.json", ws)));
            if let Some(legacy_path) = legacy {
                if legacy_path.exists() {
                    target = legacy_path;
                }
            }
        }
    }
    let data = fs::read_to_string(&target)
        .with_context(|| format!("read pidfile: {}", target.display()))?;
    let state = serde_json::from_str(&data).context("parse pidfile")?;
    Ok(state)
}

pub fn remove_pidfile(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_file(path).with_context(|| format!("remove pidfile: {}", path.display()))?;
    }
    Ok(())
}

pub fn now_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn is_pid_alive(pid: u32) -> bool {
    let mut sys = System::new();
    sys.refresh_processes();
    match sys.process(Pid::from_u32(pid)) {
        None => false,
        Some(proc) => !matches!(proc.status(), ProcessStatus::Zombie | ProcessStatus::Dead),
    }
}

pub fn send_signal(pid: u32, signal: &str) -> Result<()> {
    let status = Command::new("kill")
        .arg(signal)
        .arg(pid.to_string())
        .status()
        .with_context(|| format!("kill {} {}", signal, pid))?;
    if !status.success() {
        anyhow::bail!("failed to send {} to pid {}", signal, pid);
    }
    Ok(())
}

pub fn log_path(base_dir: &Path, ws: &str, component: &str) -> PathBuf {
    base_dir.join(ws).join(format!("{}.log", component))
}
