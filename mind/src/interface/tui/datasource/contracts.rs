use crate::interface::config::RuntimeConfig;
use crate::interface::paths;
use crate::interface::tui::app::{AppState, ContractFile};
use crate::interface::tui::datasource::DataSource;
use anyhow::Result;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

pub struct ContractsSource;

impl DataSource for ContractsSource {
    fn tick(&mut self, cfg: &RuntimeConfig, state: &mut AppState) -> Result<()> {
        let law_dir = paths::law_dir(&cfg.workspace_root);
        let roots = [
            law_dir.join("specs/cli/commands.v1.json"),
            law_dir.join("specs/cli/CLI_PUBLIC_INTERFACE.md"),
            law_dir.join("specs/cli/TUI_COCKPIT_V1.md"),
            law_dir.join("specs/graph/graph.v1.json"),
            law_dir.join("specs/providers/providers.v1.json"),
            law_dir.join("specs/control/control_plane.v1.json"),
            law_dir.join("specs/vault/vault_abi.json"),
        ];

        let mut files = Vec::new();
        for p in roots {
            if let Some(f) = describe_file(&law_dir, &p) {
                files.push(f);
            }
        }
        state.contracts.files = files;
        state.contracts.last_check = format!("{}", now_epoch());
        state.contracts.violations.clear();
        Ok(())
    }
}

fn describe_file(root: &Path, path: &PathBuf) -> Option<ContractFile> {
    let data = fs::read(path).ok()?;
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    let checksum = format!("{:016x}", hasher.finish());
    let modified_epoch = fs::metadata(path)
        .ok()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0);
    Some(ContractFile {
        path: path
            .strip_prefix(root)
            .unwrap_or(path.as_path())
            .display()
            .to_string(),
        checksum,
        modified_epoch,
    })
}

fn now_epoch() -> u64 {
    std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
