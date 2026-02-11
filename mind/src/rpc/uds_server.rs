use crate::rpc::protocol::{Request, Response};
use anyhow::{Context, Result};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};

pub async fn read_request(reader: &mut BufReader<OwnedReadHalf>) -> Result<Request> {
    let mut line = String::new();
    let _ = reader.read_line(&mut line).await?;
    if line.trim().is_empty() {
        anyhow::bail!("empty rpc request");
    }
    let req: Request = serde_json::from_str(&line).context("parse rpc request")?;
    Ok(req)
}

pub async fn write_response(writer: &mut OwnedWriteHalf, resp: &Response) -> Result<()> {
    let payload = serde_json::to_string(resp).context("serialize rpc response")?;
    writer.write_all(payload.as_bytes()).await?;
    writer.write_all(b"\n").await?;
    writer.flush().await?;
    Ok(())
}
