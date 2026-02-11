use crate::interface::config::RuntimeConfig;
use crate::rpc::protocol::{Request, Response};
use anyhow::Result;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::thread;
use std::time::Duration;

pub fn run(cfg: &RuntimeConfig, ws: &str) -> Result<()> {
    loop {
        let sock = crate::control::workspace::control_socket_path(&cfg.run_dir, ws);
        let stream = UnixStream::connect(&sock);
        let mut stream = match stream {
            Ok(s) => s,
            Err(_) => {
                thread::sleep(Duration::from_millis(500));
                continue;
            }
        };
        let req = serde_json::to_string(&Request::EventsSubscribe)?;
        stream.write_all(req.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush().ok();

        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        loop {
            line.clear();
            let n = reader.read_line(&mut line)?;
            if n == 0 {
                break;
            }
            if let Ok(resp) = serde_json::from_str::<Response>(&line) {
                match resp {
                    Response::Event { event } => {
                        println!("{}", format_event(&event));
                    }
                    _ => {}
                }
            }
        }
    }
}

fn format_event(event: &crate::rpc::protocol::Event) -> String {
    let mut base = format!("[{}] {}", event.ts, event.kind);
    if let Some(obj) = event.data.as_object() {
        let mut parts: Vec<String> = Vec::new();
        for (k, v) in obj {
            let val = if let Some(s) = v.as_str() {
                s.to_string()
            } else {
                v.to_string()
            };
            parts.push(format!("{}={}", k, val));
        }
        if !parts.is_empty() {
            base.push(' ');
            base.push_str(&parts.join(" "));
        }
    }
    base
}
