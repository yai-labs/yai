use crate::cli::config::RuntimeConfig;
use anyhow::Result;
use std::process::Command;

pub fn run(cfg: &RuntimeConfig, ws: &str) -> Result<()> {
    let sock = crate::control::workspace::control_socket_path(&cfg.run_dir, ws);
    if !sock.exists() {
        println!("daemon not running for ws={}", ws);
        return Ok(());
    }
    if in_vscode_terminal() {
        spawn_external_terminal(cfg, ws)?;
        return Ok(());
    }
    crate::cli::commands::events::run(cfg, ws)
}

pub fn spawn_external_terminal(_cfg: &RuntimeConfig, ws: &str) -> Result<()> {
    let env_prefix = build_env_prefix();
    let run = format!("{env_prefix}yai monitor --ws {ws}");

    #[cfg(target_os = "macos")]
    {
        let escaped = run.replace('"', "\\\"");
        let script = format!(
            "tell application \"iTerm2\"\n  activate\n  create window with default profile\n  tell current session of current window to write text \"{}\"\nend tell",
            escaped
        );
        let _ = Command::new("osascript").arg("-e").arg(script).spawn();
    }

    #[cfg(target_os = "linux")]
    {
        let candidates = [
            ("alacritty", vec!["-e", "sh", "-lc", run.as_str()]),
            ("kitty", vec!["sh", "-lc", run.as_str()]),
            ("gnome-terminal", vec!["--", "sh", "-lc", run.as_str()]),
            ("xterm", vec!["-e", "sh", "-lc", run.as_str()]),
        ];
        for (bin, args) in candidates {
            if Command::new(bin).args(args).spawn().is_ok() {
                break;
            }
        }
    }

    Ok(())
}

fn in_vscode_terminal() -> bool {
    std::env::var("TERM_PROGRAM")
        .map(|v| v.to_lowercase().contains("vscode"))
        .unwrap_or(false)
        || std::env::var("VSCODE_PID").is_ok()
        || std::env::var("VSCODE_GIT_IPC_HANDLE").is_ok()
}

fn build_env_prefix() -> String {
    let keys = [
        "YAI_REMOTE_ENDPOINT",
        "YAI_REMOTE_MODEL",
        "YAI_AI_PROVIDER",
        "YAI_AI_LOG",
    ];
    let mut parts = Vec::new();
    for key in keys {
        if let Ok(val) = std::env::var(key) {
            let escaped = val.replace('"', "\\\"");
            parts.push(format!("{key}=\"{escaped}\""));
        }
    }
    if parts.is_empty() {
        String::new()
    } else {
        format!("{} ", parts.join(" "))
    }
}
