use crate::cli::config::RuntimeConfig;
use crate::cli::paths;
use crate::cli::proc::{
    is_pid_alive, log_path, now_epoch, pidfile_path, read_run_state, remove_pidfile, send_signal,
    write_run_state, RunState,
};
use anyhow::{Context, Result};
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct StartOpts {
    pub build: bool,
    pub no_engine: bool,
    pub no_mind: bool,
    pub ai: bool,
    pub timeout_ms: u64,
}

pub fn ensure_ws_dir(run_dir: &Path, ws: &str) -> Result<PathBuf> {
    let dir = run_dir.join(ws);
    fs::create_dir_all(&dir).with_context(|| format!("create ws dir: {}", dir.display()))?;
    Ok(dir)
}

pub fn control_socket_path(run_dir: &Path, ws: &str) -> PathBuf {
    run_dir.join(ws).join("control.sock")
}

pub fn lock_path(run_dir: &Path, ws: &str) -> PathBuf {
    run_dir.join(ws).join("lock")
}

pub fn daemon_lock_path(run_dir: &Path, ws: &str) -> PathBuf {
    run_dir.join(ws).join("daemon.lock")
}

pub fn daemon_pid_path(run_dir: &Path, ws: &str) -> PathBuf {
    run_dir.join(ws).join("daemon.pid")
}

pub fn stopping_path(run_dir: &Path, ws: &str) -> PathBuf {
    run_dir.join(ws).join("stopping")
}

pub fn halt_path(run_dir: &Path, ws: &str) -> PathBuf {
    run_dir.join(ws).join("halt.json")
}

pub fn write_halt(run_dir: &Path, ws: &str, reason: &str) {
    let _ = fs::create_dir_all(run_dir.join(ws));
    let _ = fs::write(
        halt_path(run_dir, ws),
        serde_json::json!({ "reason": reason, "ts": now_epoch() }).to_string(),
    );
}

pub fn read_halt(run_dir: &Path, ws: &str) -> Option<String> {
    let path = halt_path(run_dir, ws);
    let data = fs::read_to_string(path).ok()?;
    serde_json::from_str::<serde_json::Value>(&data)
        .ok()
        .and_then(|v| {
            v.get("reason")
                .and_then(|r| r.as_str())
                .map(|s| s.to_string())
        })
}

pub fn clear_halt(run_dir: &Path, ws: &str) {
    let _ = fs::remove_file(halt_path(run_dir, ws));
}

pub fn acquire_lock(run_dir: &Path, ws: &str) -> Result<PathBuf> {
    let lock = lock_path(run_dir, ws);
    if lock.exists() {
        let pid = fs::read_to_string(&lock)
            .ok()
            .and_then(|v| v.trim().parse::<u32>().ok());
        if let Some(pid) = pid {
            if is_pid_alive(pid) {
                anyhow::bail!("lock already held by pid {}", pid);
            }
        }
        fs::remove_file(&lock).ok();
    }
    let pid = std::process::id();
    fs::write(&lock, pid.to_string()).with_context(|| format!("write lock: {}", lock.display()))?;
    Ok(lock)
}

pub fn acquire_daemon_lock(run_dir: &Path, ws: &str) -> Result<PathBuf> {
    let lock = daemon_lock_path(run_dir, ws);
    if lock.exists() {
        let pid = fs::read_to_string(&lock)
            .ok()
            .and_then(|v| v.trim().parse::<u32>().ok());
        if let Some(pid) = pid {
            if is_pid_alive(pid) {
                anyhow::bail!("daemon lock already held by pid {}", pid);
            }
        }
        fs::remove_file(&lock).ok();
    }
    let pid = std::process::id();
    fs::write(&lock, pid.to_string())
        .with_context(|| format!("write daemon lock: {}", lock.display()))?;
    Ok(lock)
}

pub fn release_lock(lock_path: &Path) {
    let _ = fs::remove_file(lock_path);
}

pub fn read_state(cfg: &RuntimeConfig, ws: &str) -> Option<crate::cli::proc::RunState> {
    let pidfile = pidfile_path(&cfg.run_dir, ws);
    if !pidfile.exists() {
        return None;
    }
    read_run_state(&pidfile).ok()
}

pub fn start_stack(
    cfg: &RuntimeConfig,
    ws: &str,
    opts: &StartOpts,
    bus: &crate::control::events::EventBus,
) -> Result<RunState> {
    let _ = std::fs::create_dir_all(cfg.run_dir.join(ws));
    let socket_path = paths::ws_socket_path(&cfg.socket_path, ws);
    clear_halt(&cfg.run_dir, ws);

    if opts.build {
        let core_dir = paths::core_dir(&cfg.workspace_root);
        let status = Command::new("make")
            .arg("package")
            .current_dir(&core_dir)
            .status()
            .with_context(|| format!("make package in {}", core_dir.display()))?;
        if !status.success() {
            anyhow::bail!("build failed: make package");
        }

        if !opts.no_mind {
            let mind_dir = paths::mind_dir(&cfg.workspace_root);
            let status = Command::new("cargo")
                .arg("build")
                .arg("--release")
                .current_dir(&mind_dir)
                .status()
                .with_context(|| format!("cargo build in {}", mind_dir.display()))?;
            if !status.success() {
                anyhow::bail!("build failed: yai");
            }
        }
    }

    if socket_path.exists() {
        let _ = fs::remove_file(&socket_path);
    }

    let pidfile = pidfile_path(&cfg.run_dir, ws);
    if pidfile.exists() {
        let _ = stop_stack(cfg, ws, true, bus);
    }

    let boot_log_path = log_path(&cfg.run_dir, ws, "boot");
    let boot_log = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&boot_log_path)
        .with_context(|| format!("open log: {}", boot_log_path.display()))?;

    let mut boot_cmd = Command::new(&cfg.yai_boot);
    #[cfg(unix)]
    detach_child(&mut boot_cmd);
    boot_cmd.arg("--ws").arg(ws).arg("--raid");
    boot_cmd.stdout(Stdio::piped());
    boot_cmd.stderr(Stdio::from(boot_log.try_clone()?));

    let mut boot_child = boot_cmd.spawn().context("spawn yai-boot")?;
    let boot_pid = boot_child.id();

    let stdout = boot_child
        .stdout
        .take()
        .context("capture yai-boot stdout")?;

    let (tx, rx) = mpsc::channel::<String>();
    let mut boot_log_thread = boot_log.try_clone()?;
    thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();
        loop {
            line.clear();
            let n = match reader.read_line(&mut line) {
                Ok(n) => n,
                Err(_) => break,
            };
            if n == 0 {
                break;
            }
            let _ = boot_log_thread.write_all(line.as_bytes());
            let _ = tx.send(line.clone());
        }
    });

    let timeout = Duration::from_millis(opts.timeout_ms);
    let deadline = Instant::now() + timeout;
    let ready_tag = format!("YAI_BOOT_OK ws={}", ws);
    let mut ready = false;

    while Instant::now() < deadline {
        let remaining = deadline.saturating_duration_since(Instant::now());
        match rx.recv_timeout(remaining) {
            Ok(line) => {
                if line.contains(&ready_tag) {
                    ready = true;
                    break;
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if let Ok(Some(status)) = boot_child.try_wait() {
                    anyhow::bail!("yai-boot exited before readiness: {}", status);
                }
            }
            Err(_) => break,
        }
    }

    if !ready {
        let _ = boot_child.kill();
        anyhow::bail!("timeout waiting for yai-boot readiness");
    }

    let kernel_pid;
    let mut engine_pid = None;
    let mut mind_pid = None;

    let kernel_log_path = log_path(&cfg.run_dir, ws, "kernel");
    let kernel_log = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&kernel_log_path)
        .with_context(|| format!("open log: {}", kernel_log_path.display()))?;
    let mut kernel_cmd = Command::new(&cfg.yai_kernel);
    kernel_cmd.current_dir(&cfg.workspace_root);
    #[cfg(unix)]
    detach_child(&mut kernel_cmd);
    kernel_cmd.arg("--ws").arg(ws);
    kernel_cmd.env("YAI_WORKSPACE_ID", ws);
    kernel_cmd.env("YAI_RUNTIME_SOCKET", socket_path.display().to_string());
    kernel_cmd
        .stdout(Stdio::from(kernel_log.try_clone()?))
        .stderr(Stdio::from(kernel_log));
    let mut kernel_child = kernel_cmd.spawn().context("spawn yai-kernel")?;
    kernel_pid = Some(kernel_child.id());
    let _ = bus.emit(
        "proc_started",
        serde_json::json!({ "proc": "boot", "pid": boot_pid, "pgid": boot_pid }),
    );
    let _ = bus.emit(
        "proc_started",
        serde_json::json!({ "proc": "kernel", "pid": kernel_child.id(), "pgid": boot_pid }),
    );

    // wait for runtime socket
    let timeout = Duration::from_millis(opts.timeout_ms);
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if socket_path.exists() {
            break;
        }
        if let Ok(Some(status)) = kernel_child.try_wait() {
            anyhow::bail!("yai-kernel exited before runtime socket: {}", status);
        }
        thread::sleep(Duration::from_millis(50));
    }
    if !socket_path.exists() {
        let _ = kernel_child.kill();
        anyhow::bail!(
            "timeout waiting for runtime socket {}",
            socket_path.display()
        );
    }

    if !opts.no_engine {
        let engine_log_path = log_path(&cfg.run_dir, ws, "engine");
        let engine_log = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&engine_log_path)
            .with_context(|| format!("open log: {}", engine_log_path.display()))?;
        let mut cmd = Command::new(&cfg.yai_engine);
        #[cfg(unix)]
        detach_child(&mut cmd);
        cmd.arg(ws)
            .env("YAI_WORKSPACE_ID", ws)
            .env("YAI_RUNTIME_SOCKET", socket_path.display().to_string())
            .stdout(Stdio::from(engine_log.try_clone()?))
            .stderr(Stdio::from(engine_log));
        let child = cmd.spawn().context("spawn yai-engine")?;
        engine_pid = Some(child.id());
        let _ = bus.emit(
            "proc_started",
            serde_json::json!({ "proc": "engine", "pid": child.id(), "pgid": boot_pid }),
        );
    }

    if !opts.no_mind {
        let mind_log_path = log_path(&cfg.run_dir, ws, "mind");
        let mind_log = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&mind_log_path)
            .with_context(|| format!("open log: {}", mind_log_path.display()))?;
        let built_yai = paths::mind_dir(&cfg.workspace_root)
            .join("target")
            .join("release")
            .join("yai");
        let mind_exec = if opts.build && built_yai.exists() {
            built_yai
        } else {
            cfg.yai_mind.clone()
        };
        let mut cmd = Command::new(&mind_exec);
        #[cfg(unix)]
        detach_child(&mut cmd);
        cmd.arg("mind");
        if opts.ai {
            cmd.env("YAI_AI_LOG", "1");
        }
        cmd.env("YAI_WORKSPACE_ID", ws);
        cmd.stdout(Stdio::from(mind_log.try_clone()?))
            .stderr(Stdio::from(mind_log));
        let child = cmd.spawn().context("spawn yai")?;
        mind_pid = Some(child.id());
        let _ = bus.emit(
            "proc_started",
            serde_json::json!({ "proc": "mind", "pid": child.id(), "pgid": boot_pid }),
        );
    }

    let pgid = Some(boot_pid);
    #[cfg(unix)]
    {
        setpgid_parent(boot_pid, boot_pid);
        if let Some(pid) = kernel_pid {
            setpgid_parent(pid, boot_pid);
        }
        if let Some(pid) = engine_pid {
            setpgid_parent(pid, boot_pid);
        }
        if let Some(pid) = mind_pid {
            setpgid_parent(pid, boot_pid);
        }
    }

    let state = RunState {
        ws: ws.to_string(),
        boot_pid: Some(boot_pid),
        kernel_pid,
        engine_pid,
        mind_pid,
        pgid,
        socket_path: socket_path.display().to_string(),
        artifacts_root: cfg.artifacts_root.display().to_string(),
        started_at_epoch: now_epoch(),
    };
    write_run_state(&pidfile, &state)?;

    // supervisor: if kernel exits, tear down session
    let cfg_clone = cfg.clone();
    let ws_clone = ws.to_string();
    let bus_clone = bus.clone();
    thread::spawn(move || {
        let _ = kernel_child.wait();
        let _ = stop_stack(&cfg_clone, &ws_clone, true, &bus_clone);
    });

    Ok(state)
}

#[cfg(unix)]
fn detach_child(cmd: &mut Command) {
    unsafe {
        cmd.pre_exec(|| {
            libc::setpgid(0, 0);
            Ok(())
        });
    }
}

#[cfg(unix)]
fn setpgid_parent(pid: u32, pgid: u32) {
    unsafe {
        libc::setpgid(pid as i32, pgid as i32);
    }
}

pub fn stop_stack(
    cfg: &RuntimeConfig,
    ws: &str,
    force: bool,
    bus: &crate::control::events::EventBus,
) -> Result<()> {
    let stopping = stopping_path(&cfg.run_dir, ws);
    if !stopping.exists() {
        let _ = bus.emit("ws_down_started", serde_json::json!({ "ws": ws }));
        let _ = std::fs::write(&stopping, "1");
    }

    let socket_path = paths::ws_socket_path(&cfg.socket_path, ws);
    let pidfile = pidfile_path(&cfg.run_dir, ws);
    if pidfile.exists() {
        let state = read_run_state(&pidfile).ok();
        if let Some(state) = state {
            #[cfg(unix)]
            if let Some(pgid) = state.pgid {
                unsafe {
                    libc::kill(-(pgid as i32), libc::SIGTERM);
                }
            }
            for pid in [
                state.mind_pid,
                state.engine_pid,
                state.kernel_pid,
                state.boot_pid,
            ] {
                if let Some(pid) = pid {
                    if is_pid_alive(pid) {
                        let _ = send_signal(pid, "-TERM");
                    }
                }
            }

            thread::sleep(Duration::from_secs(2));

            if force {
                #[cfg(unix)]
                if let Some(pgid) = state.pgid {
                    unsafe {
                        libc::kill(-(pgid as i32), libc::SIGKILL);
                    }
                }
                for pid in [
                    state.mind_pid,
                    state.engine_pid,
                    state.kernel_pid,
                    state.boot_pid,
                ] {
                    if let Some(pid) = pid {
                        if is_pid_alive(pid) {
                            let _ = send_signal(pid, "-KILL");
                        }
                    }
                }
            }
        }

        remove_pidfile(&pidfile)?;
    }

    if socket_path.exists() {
        let _ = fs::remove_file(&socket_path);
    }
    let _ = std::fs::remove_file(&stopping);
    let _ = bus.emit("ws_down_complete", serde_json::json!({ "ws": ws }));
    Ok(())
}
