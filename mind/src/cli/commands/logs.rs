use crate::cli::config::RuntimeConfig;
use crate::cli::proc::log_path;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

pub fn run(
    cfg: &RuntimeConfig,
    ws: &str,
    component: &str,
    follow: bool,
    tail_lines: usize,
) -> Result<()> {
    let path = log_path(&cfg.run_dir, ws, component);
    if !path.exists() {
        anyhow::bail!("log not found: {}", path.display());
    }

    if follow {
        follow_file_prefixed(&path, None)?;
        return Ok(());
    }

    let mut content = String::new();
    File::open(&path)
        .with_context(|| format!("open log: {}", path.display()))?
        .read_to_string(&mut content)
        .with_context(|| format!("read log: {}", path.display()))?;

    let lines: Vec<&str> = content.lines().collect();
    let start = lines.len().saturating_sub(tail_lines);
    for line in &lines[start..] {
        println!("{}", line);
    }
    Ok(())
}

pub fn follow_components(cfg: &RuntimeConfig, ws: &str, components: &[&str]) -> Result<()> {
    let mut paths: Vec<(String, PathBuf)> = Vec::new();
    for component in components {
        let path = log_path(&cfg.run_dir, ws, component);
        if path.exists() {
            paths.push((component.to_string(), path));
        }
    }
    if paths.is_empty() {
        anyhow::bail!("no logs found for ws {}", ws);
    }
    let mut iter = paths.into_iter();
    let (first_label, first_path) = iter.next().unwrap();
    for (label, path) in iter {
        let label = label.clone();
        let path = path.clone();
        thread::spawn(move || {
            let _ = follow_file_prefixed(&path, Some(&label));
        });
    }
    follow_file_prefixed(&first_path, Some(&first_label))
}

fn follow_file_prefixed(path: &std::path::Path, prefix: Option<&str>) -> Result<()> {
    let mut file = File::open(path).with_context(|| format!("open log: {}", path.display()))?;
    file.seek(SeekFrom::End(0))?;
    let mut reader = BufReader::new(file);
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            thread::sleep(Duration::from_millis(200));
            continue;
        }
        if let Some(pfx) = prefix {
            if pfx == "kernel" && is_kernel_tree_noise(&line) {
                continue;
            }
            print!("[{}] {}", pfx, line);
        } else {
            print!("{}", line);
        }
    }
}

fn is_kernel_tree_noise(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("[FILE]")
        || trimmed.starts_with("[FOLDER]")
        || trimmed.starts_with("[KERNEL]") && trimmed.contains("Awaiting commands")
}
