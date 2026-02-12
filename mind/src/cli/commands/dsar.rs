use crate::cli::config::RuntimeConfig;
use crate::transport::rpc::protocol::{Request, Response};
use crate::transport::rpc::uds_client;
use anyhow::Result;

pub fn request(cfg: &RuntimeConfig, ws: &str, request_type: String, subject: String) -> Result<()> {
    let req = Request::DsarRequest {
        request_type,
        subject_ref: subject,
    };
    match uds_client::send_request(&cfg.run_dir, ws, &req)? {
        Response::DsarCreated { request } => {
            println!(
                "request_id={} type={} subject={} status={:?}",
                request.request_id, request.request_type, request.subject_ref, request.status
            );
        }
        Response::Error { message } => println!("error: {}", message),
        other => println!("unexpected response: {:?}", other),
    }
    Ok(())
}

pub fn status(cfg: &RuntimeConfig, ws: &str, request_id: String) -> Result<()> {
    let req = Request::DsarStatus { request_id };
    match uds_client::send_request(&cfg.run_dir, ws, &req)? {
        Response::DsarState { request } => {
            if let Some(r) = request {
                println!(
                    "request_id={} type={} subject={} status={:?}",
                    r.request_id, r.request_type, r.subject_ref, r.status
                );
            } else {
                println!("not found");
            }
        }
        Response::Error { message } => println!("error: {}", message),
        other => println!("unexpected response: {:?}", other),
    }
    Ok(())
}

pub fn execute(cfg: &RuntimeConfig, ws: &str, request_id: String) -> Result<()> {
    let req = Request::DsarExecute { request_id };
    match uds_client::send_request(&cfg.run_dir, ws, &req)? {
        Response::DsarExecuted { request } => {
            println!(
                "request_id={} type={} subject={} status={:?}",
                request.request_id, request.request_type, request.subject_ref, request.status
            );
        }
        Response::Error { message } => println!("error: {}", message),
        other => println!("unexpected response: {:?}", other),
    }
    Ok(())
}
