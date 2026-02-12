use crate::control::workspace;
use crate::transport::rpc::protocol::{Request, Response};
use anyhow::{Context, Result};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;

pub fn send_request(run_dir: &Path, ws: &str, req: &Request) -> Result<Response> {
    let sock = workspace::control_socket_path(run_dir, ws);
    let mut stream = UnixStream::connect(&sock)
        .with_context(|| format!("connect control socket: {}", sock.display()))?;
    let payload = serde_json::to_string(req).context("serialize rpc request")?;
    stream
        .write_all(payload.as_bytes())
        .context("write rpc request")?;
    stream.write_all(b"\n").context("write rpc newline")?;
    stream.flush().ok();

    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line).context("read rpc response")?;
    let resp: Response = serde_json::from_str(&line).context("parse rpc response")?;
    Ok(resp)
}
