use crate::rpc::protocol::{Request, Response};
use crate::rpc::uds_client;
use crate::interface::config::RuntimeConfig;
use anyhow::Result;

pub fn discover(cfg: &RuntimeConfig, ws: &str) -> Result<()> {
    let endpoint = std::env::var("YAI_REMOTE_ENDPOINT").ok();
    let model = std::env::var("YAI_REMOTE_MODEL").ok();
    let req = Request::ProvidersDiscover { endpoint, model };
    match uds_client::send_request(&cfg.run_dir, ws, &req)? {
        Response::Providers { items } => {
            for p in items {
                println!("candidate: {} {} {} {}", p.id, p.endpoint, p.model, format!("{:?}", p.trust_state).to_lowercase());
            }
        }
        Response::Error { message } => println!("error: {}", message),
        other => println!("unexpected response: {:?}", other),
    }
    Ok(())
}

pub fn list(cfg: &RuntimeConfig, ws: &str) -> Result<()> {
    match uds_client::send_request(&cfg.run_dir, ws, &Request::ProvidersList)? {
        Response::Providers { items } => {
            for p in items {
                println!(
                    "provider: {} {} {} {}",
                    p.id,
                    p.endpoint,
                    p.model,
                    format!("{:?}", p.trust_state).to_lowercase()
                );
            }
        }
        Response::Error { message } => println!("error: {}", message),
        other => println!("unexpected response: {:?}", other),
    }
    Ok(())
}

pub fn pair(cfg: &RuntimeConfig, ws: &str, id: String, endpoint: String, model: String) -> Result<()> {
    let req = Request::ProvidersPair { id, endpoint, model };
    match uds_client::send_request(&cfg.run_dir, ws, &req)? {
        Response::ProvidersOk => println!("paired"),
        Response::Error { message } => println!("error: {}", message),
        other => println!("unexpected response: {:?}", other),
    }
    Ok(())
}

pub fn attach(cfg: &RuntimeConfig, ws: &str, id: String, model: Option<String>) -> Result<()> {
    let req = Request::ProvidersAttach { id, model };
    match uds_client::send_request(&cfg.run_dir, ws, &req)? {
        Response::ProvidersOk => println!("attached"),
        Response::Error { message } => println!("error: {}", message),
        other => println!("unexpected response: {:?}", other),
    }
    Ok(())
}

pub fn detach(cfg: &RuntimeConfig, ws: &str) -> Result<()> {
    match uds_client::send_request(&cfg.run_dir, ws, &Request::ProvidersDetach)? {
        Response::ProvidersOk => println!("detached"),
        Response::Error { message } => println!("error: {}", message),
        other => println!("unexpected response: {:?}", other),
    }
    Ok(())
}

pub fn revoke(cfg: &RuntimeConfig, ws: &str, id: String) -> Result<()> {
    let req = Request::ProvidersRevoke { id };
    match uds_client::send_request(&cfg.run_dir, ws, &req)? {
        Response::ProvidersOk => println!("revoked"),
        Response::Error { message } => println!("error: {}", message),
        other => println!("unexpected response: {:?}", other),
    }
    Ok(())
}

pub fn status(cfg: &RuntimeConfig, ws: &str) -> Result<()> {
    match uds_client::send_request(&cfg.run_dir, ws, &Request::ProvidersStatus)? {
        Response::ProviderStatus { active } => {
            if let Some(p) = active {
                println!(
                    "active: {} {} {} {}",
                    p.id,
                    p.endpoint,
                    p.model,
                    format!("{:?}", p.trust_state).to_lowercase()
                );
            } else {
                println!("active: none");
            }
        }
        Response::Error { message } => println!("error: {}", message),
        other => println!("unexpected response: {:?}", other),
    }
    Ok(())
}
