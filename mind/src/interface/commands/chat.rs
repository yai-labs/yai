use crate::interface::config::RuntimeConfig;
use crate::rpc::protocol::{Request, Response};
use crate::rpc::uds_client;
use anyhow::Result;

pub fn list(cfg: &RuntimeConfig, ws: &str) -> Result<()> {
    match uds_client::send_request(&cfg.run_dir, ws, &Request::ChatSessionsList)? {
        Response::ChatSessions { items, selected } => {
            for s in items {
                let marker = if selected.as_deref() == Some(s.id.as_str()) {
                    "*"
                } else {
                    " "
                };
                println!(
                    "{} {} created={} last={} {}",
                    marker,
                    s.id,
                    s.created_ts_ms,
                    s.last_ts_ms,
                    s.title.unwrap_or_default()
                );
            }
            Ok(())
        }
        Response::Error { message } => anyhow::bail!("{message}"),
        other => anyhow::bail!("unexpected response: {other:?}"),
    }
}

pub fn new(cfg: &RuntimeConfig, ws: &str, title: Option<String>) -> Result<()> {
    match uds_client::send_request(&cfg.run_dir, ws, &Request::ChatSessionNew { title })? {
        Response::ChatSession { session } => {
            println!("{}", session.id);
            Ok(())
        }
        Response::Error { message } => anyhow::bail!("{message}"),
        other => anyhow::bail!("unexpected response: {other:?}"),
    }
}

pub fn select(cfg: &RuntimeConfig, ws: &str, session_id: &str) -> Result<()> {
    match uds_client::send_request(
        &cfg.run_dir,
        ws,
        &Request::ChatSessionSelect {
            session_id: session_id.to_string(),
        },
    )? {
        Response::ChatSessions { .. } => {
            println!("selected: {session_id}");
            Ok(())
        }
        Response::Error { message } => anyhow::bail!("{message}"),
        other => anyhow::bail!("unexpected response: {other:?}"),
    }
}

pub fn history(cfg: &RuntimeConfig, ws: &str, session_id: Option<String>) -> Result<()> {
    match uds_client::send_request(&cfg.run_dir, ws, &Request::ChatHistory { session_id })? {
        Response::ChatHistory { session_id, items } => {
            println!("# session={session_id}");
            for m in items {
                println!("[{} {:?}] {}", m.ts_ms, m.role, m.content);
            }
            Ok(())
        }
        Response::Error { message } => anyhow::bail!("{message}"),
        other => anyhow::bail!("unexpected response: {other:?}"),
    }
}

pub fn send(
    cfg: &RuntimeConfig,
    ws: &str,
    session_id: Option<String>,
    stream: bool,
    text: &str,
) -> Result<()> {
    let req = Request::ChatSend {
        session_id,
        text: text.to_string(),
        stream,
    };
    match uds_client::send_request(&cfg.run_dir, ws, &req)? {
        Response::ChatSend { message } => {
            println!("[{:?}] {}", message.role, message.content);
            Ok(())
        }
        Response::Error { message } => anyhow::bail!("{message}"),
        other => anyhow::bail!("unexpected response: {other:?}"),
    }
}
