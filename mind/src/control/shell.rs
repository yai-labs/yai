use anyhow::{Context, Result};
use std::collections::HashSet;
use std::process::Stdio;
use std::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::{Child, Command};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Capability {
    ShellExec,
    ShellPty,
}

#[derive(Debug, Clone)]
pub struct ShellPolicy {
    pub lockdown: bool,
    pub armed: bool,
    pub allow: HashSet<Capability>,
}

impl Default for ShellPolicy {
    fn default() -> Self {
        let mut allow = HashSet::new();
        allow.insert(Capability::ShellExec);
        Self {
            lockdown: false,
            armed: false,
            allow,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExecResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub struct ShellService {
    policy: Mutex<ShellPolicy>,
}

impl Default for ShellService {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellService {
    pub fn new() -> Self {
        Self {
            policy: Mutex::new(ShellPolicy::default()),
        }
    }

    pub fn set_policy(&self, policy: ShellPolicy) {
        *self.policy.lock().unwrap() = policy;
    }

    pub async fn exec(&self, cmd: &str, args: &[String], cwd: Option<&str>) -> Result<ExecResult> {
        let policy = self.policy.lock().unwrap().clone();
        if policy.lockdown || !policy.allow.contains(&Capability::ShellExec) {
            anyhow::bail!("shell.exec denied (lockdown/capability)");
        }

        let mut c = Command::new(cmd);
        c.args(args);
        if let Some(dir) = cwd {
            c.current_dir(dir);
        }
        let output = c
            .output()
            .await
            .with_context(|| format!("shell.exec failed: {cmd}"))?;
        Ok(ExecResult {
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }

    pub async fn pty_open(&self, shell: &str, cwd: Option<&str>) -> Result<Child> {
        let policy = self.policy.lock().unwrap().clone();
        if !policy.armed || policy.lockdown || !policy.allow.contains(&Capability::ShellPty) {
            anyhow::bail!("shell.pty denied (arming/lockdown/capability)");
        }

        let mut cmd = Command::new(shell);
        if let Some(dir) = cwd {
            cmd.current_dir(dir);
        }
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let child = cmd
            .spawn()
            .with_context(|| format!("spawn pty shell: {shell}"))?;
        Ok(child)
    }
}

pub async fn pty_write(child: &mut Child, data: &[u8]) -> Result<()> {
    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(data).await.context("pty write")?;
    }
    Ok(())
}

pub async fn pty_read_once(child: &mut Child) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    if let Some(stdout) = child.stdout.as_mut() {
        let mut buf = [0u8; 4096];
        let n = stdout.read(&mut buf).await.context("pty read")?;
        out.extend_from_slice(&buf[..n]);
    }
    Ok(out)
}
