use crate::cli::config::RuntimeConfig;
use crate::cli::proc::log_path;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
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
        follow_file(&path)?;
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

fn follow_file(path: &std::path::Path) -> Result<()> {
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
        print!("{}", line);
    }
}
