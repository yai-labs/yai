//! mind/src/transport/rpc/uds_server.rs
//! UDS RPC server — STRICT EnvelopeV1 (fallback legacy ONLY to return protocol_invalid)

use crate::transport::rpc::protocol::{Request, Response, RPC_PROTOCOL_VERSION};
use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use serde_json::Value;
use std::{future::Future, path::Path, pin::Pin, sync::Arc};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{UnixListener, UnixStream};

use yx_protocol::{ClientInfo, RequestEnvelopeV1, ResponseEnvelopeV1, Role, RpcError};

pub struct RpcInboundV1 {
    pub v: u8,
    pub trace_id: String,
    pub ws_id: String,
    pub arming: bool,
    pub role: Role,
    pub client: ClientInfo,
    pub request: Request,
}

// ---------------------------
// Handler trait (no extra deps)
// ---------------------------

pub type BoxFut<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub trait RpcHandler: Send + Sync + 'static {
    fn handle<'a>(&'a self, inbound: RpcInboundV1) -> BoxFut<'a, Result<Response>>;
}

// ---------------------------
// Server loop
// ---------------------------

pub async fn serve<P: AsRef<Path>>(socket_path: P, handler: Arc<dyn RpcHandler>) -> Result<()> {
    let sock = socket_path.as_ref();

    // remove stale socket
    if tokio::fs::metadata(sock).await.is_ok() {
        let _ = tokio::fs::remove_file(sock).await;
    }

    let listener =
        UnixListener::bind(sock).with_context(|| format!("bind uds socket: {}", sock.display()))?;

    loop {
        let (stream, _addr) = listener.accept().await.context("uds accept")?;
        let h = handler.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_stream(stream, h).await {
                // non facciamo crashare tutto: log minimale
                eprintln!("[uds_server] connection error: {e:#}");
            }
        });
    }
}

async fn handle_stream(stream: UnixStream, handler: Arc<dyn RpcHandler>) -> Result<()> {
    let (read_half, mut write_half) = stream.into_split();
    let mut reader = BufReader::new(read_half);

    loop {
        match read_request_v1(&mut reader).await {
            Ok(inbound) => {
                let trace_id = inbound.trace_id.clone();
                let ws_id = inbound.ws_id.clone();
                let v = inbound.v;

                let resp = handler
                    .handle(inbound)
                    .await
                    .context("rpc handler failed")?;

                write_response_v1(&mut write_half, v, &trace_id, &ws_id, &resp).await?;
            }
            Err(e) => {
                if is_eof(&e) {
                    return Ok(());
                }

                // se l’errore porta già un envelope di risposta, lo scriviamo e chiudiamo
                if let Some((v, trace_id, ws_id, rpc_err)) = e
                    .downcast_ref::<ProtocolReject>()
                    .map(|r| (r.v, r.trace_id.clone(), r.ws_id.clone(), r.error.clone()))
                {
                    let _ = write_error_v1(&mut write_half, v, &trace_id, &ws_id, &rpc_err).await;
                    return Ok(());
                }

                return Err(e);
            }
        }
    }
}

fn is_eof(e: &anyhow::Error) -> bool {
    let s = e.to_string();
    s.contains("rpc eof") || s.contains("early eof") || s.contains("connection reset")
}

// ---------------------------
// Robust parsing helpers
// ---------------------------

#[derive(Debug, Deserialize)]
struct RpcRequestEnvelopeLoose {
    // accetta anche "version"
    #[serde(alias = "version")]
    pub v: serde_json::Value,

    #[serde(default)]
    pub arming: bool,

    #[serde(default)]
    pub role: Option<String>,

    // accetta ws_id, wsId, workspace_id
    #[serde(default, alias = "wsId", alias = "workspace_id")]
    pub ws_id: Option<String>,

    pub request: serde_json::Value,
}

fn parse_version(v: &serde_json::Value) -> Option<u8> {
    match v {
        serde_json::Value::Number(n) => n.as_u64().and_then(|x| u8::try_from(x).ok()),
        serde_json::Value::String(s) => s.parse::<u8>().ok(),
        _ => None,
    }
}

fn preview_json(v: &serde_json::Value, max: usize) -> String {
    let s = serde_json::to_string(v).unwrap_or_else(|_| "<unserializable>".to_string());
    if s.len() > max {
        format!("{}…", &s[..max])
    } else {
        s
    }
}

fn pascal_to_snake(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for (i, ch) in s.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i != 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

fn pascal_to_dot(s: &str) -> String {
    pascal_to_snake(s).replace('_', ".")
}

fn try_parse_request_candidates(candidates: Vec<serde_json::Value>) -> Result<Request> {
    for c in candidates {
        if let Ok(req) = serde_json::from_value::<Request>(c) {
            return Ok(req);
        }
    }
    anyhow::bail!("no compatible request encoding matched")
}

/// Prova a deserializzare `Request` con più forme tollerate:
/// 1) parse diretto (il formato “canonico” di Request)
/// 2) request come stringa
/// 3) request object con tag alternativi (op/kind/cmd/name/type) normalizzati
fn parse_request_compat(v: &serde_json::Value) -> Result<Request> {
    // 1) Canonico (il formato attuale server)
    if let Ok(req) = serde_json::from_value::<Request>(v.clone()) {
        return Ok(req);
    }

    // 2) Stringa (se mai arrivasse)
    if let Some(s) = v.as_str() {
        return serde_json::from_value::<Request>(serde_json::Value::String(s.to_string()))
            .context("parse request from string");
    }

    // 3) Object
    if let Some(obj) = v.as_object() {
        // 3a) Externally-tagged enum: { "VariantName": { ... } }
        if obj.len() == 1 {
            let (variant, inner) = obj.iter().next().unwrap();

            // se inner non è object, proviamo comunque a usarlo come payload
            let inner_obj = inner.as_object();

            // proviamo più normalizzazioni del nome variante
            let mut tags = vec![
                variant.to_string(),
                variant.to_lowercase(),
                pascal_to_snake(variant),
                pascal_to_dot(variant),
            ];
            tags.dedup();

            let mut candidates = Vec::new();

            for tag in tags {
                // candidato A: {"type": tag, ...inner_fields} (flatten)
                if let Some(inner_obj) = inner_obj {
                    let mut flat = serde_json::Map::new();
                    flat.insert("type".to_string(), serde_json::Value::String(tag.clone()));
                    for (k, val) in inner_obj {
                        flat.insert(k.clone(), val.clone());
                    }
                    candidates.push(serde_json::Value::Object(flat));
                }

                // candidato B: {"type": tag, "payload": inner}
                candidates.push(serde_json::json!({ "type": tag, "payload": inner }));

                // candidato C: {"type": tag, "data": inner}
                candidates.push(serde_json::json!({ "type": tag, "data": inner }));

                // candidato D: {"op": tag, ...} (se lato server hai tag diversi)
                if let Some(inner_obj) = inner_obj {
                    let mut op_flat = serde_json::Map::new();
                    op_flat.insert("op".to_string(), serde_json::Value::String(tag.clone()));
                    for (k, val) in inner_obj {
                        op_flat.insert(k.clone(), val.clone());
                    }
                    candidates.push(serde_json::Value::Object(op_flat));
                }
            }

            if let Ok(req) = try_parse_request_candidates(candidates) {
                return Ok(req);
            }
        }

        // 3b) Object con tag esplicito (type/op/kind/cmd/name)
        let tag = obj
            .get("type")
            .or_else(|| obj.get("op"))
            .or_else(|| obj.get("kind"))
            .or_else(|| obj.get("cmd"))
            .or_else(|| obj.get("name"))
            .and_then(|x| x.as_str());

        if let Some(tag) = tag {
            let tag_val = serde_json::Value::String(tag.to_string());
            let mut normalized = obj.clone();
            for k in ["type", "op", "kind", "cmd", "name"] {
                normalized.entry(k.to_string()).or_insert(tag_val.clone());
            }
            let norm_val = serde_json::Value::Object(normalized);
            if let Ok(req) = serde_json::from_value::<Request>(norm_val) {
                return Ok(req);
            }
        }
    }

    anyhow::bail!("unsupported request shape: {}", preview_json(v, 240));
}

#[derive(Debug)]
struct ProtocolReject {
    v: u8,
    trace_id: String,
    ws_id: String,
    error: RpcError,
}

impl std::fmt::Display for ProtocolReject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "protocol reject: {}", self.error.message)
    }
}
impl std::error::Error for ProtocolReject {}

fn ws_missing(trace_id: String) -> ProtocolReject {
    let trace_id_opt = Some(trace_id.clone());
    ProtocolReject {
        v: RPC_PROTOCOL_VERSION,
        trace_id,
        ws_id: "".to_string(),
        error: RpcError {
            code: Some("ws_missing".to_string()),
            message: "ws_id missing".to_string(),
            detail: None,
            trace_id: trace_id_opt,
            ws_id: Some("".to_string()),
        },
    }
}

fn handshake_required(trace_id: String, ws_id: String) -> ProtocolReject {
    ProtocolReject {
        v: RPC_PROTOCOL_VERSION,
        trace_id: trace_id.clone(),
        ws_id: ws_id.clone(),
        error: RpcError {
            code: Some("handshake_required".to_string()),
            message: "client missing rpc.v1 capability".to_string(),
            detail: Some(serde_json::json!({
                "required_capability": "rpc.v1"
            })),
            trace_id: Some(trace_id),
            ws_id: Some(ws_id),
        },
    }
}

fn protocol_invalid() -> ProtocolReject {
    ProtocolReject {
        v: RPC_PROTOCOL_VERSION,
        trace_id: "unknown".to_string(),
        ws_id: "".to_string(),
        error: RpcError {
            code: Some("protocol_invalid".to_string()),
            message: "invalid rpc envelope".to_string(),
            detail: None,
            trace_id: None,
            ws_id: None,
        },
    }
}

// ---------------------------
// Public read/write API
// ---------------------------
pub async fn read_request_v1(reader: &mut BufReader<OwnedReadHalf>) -> Result<RpcInboundV1> {
    let mut line = String::new();
    let n = reader.read_line(&mut line).await.context("read rpc line")?;
    if n == 0 {
        anyhow::bail!("rpc eof");
    }

    let raw = line.trim_end();
    if raw.is_empty() {
        return Err(anyhow!("empty rpc request"));
    }

    // STRICT: parse EnvelopeV1
    let env: RequestEnvelopeV1<Value> = match serde_json::from_str(raw) {
        Ok(v) => v,
        Err(_) => {
            // fallback legacy parse SOLO per poter rispondere protocol_invalid
            let _legacy_req_attempt: Result<Request> =
                serde_json::from_str(raw).map_err(|e| e.into());
            return Err(ProtocolReject::protocol_invalid().into());
        }
    };

    let v = u8::try_from(env.v).unwrap_or(0);
    if v != RPC_PROTOCOL_VERSION {
        return Err(ProtocolReject {
            v: RPC_PROTOCOL_VERSION,
            trace_id: env.trace_id.clone(),
            ws_id: env.ws_id.clone(),
            error: RpcError {
                code: Some("protocol_mismatch".to_string()),
                message: format!("client v{} != server v{}", v, RPC_PROTOCOL_VERSION),
                detail: None,
                trace_id: Some(env.trace_id.clone()),
                ws_id: Some(env.ws_id.clone()),
            },
        }
        .into());
    }

    if env.ws_id.trim().is_empty() {
        return Err(ws_missing(env.trace_id).into());
    }

    // client mandatory + must include rpc.v1
    let has_rpc_v1 = env.client.capabilities.iter().any(|c| c == "rpc.v1");
    if env.client.client_kind.trim().is_empty()
        || env.client.client_version.trim().is_empty()
        || !has_rpc_v1
    {
        return Err(handshake_required(env.trace_id, env.ws_id).into());
    }

    // request MUST be domain enum
    let req: Request = serde_json::from_value(env.request).map_err(|e| ProtocolReject {
        v: RPC_PROTOCOL_VERSION,
        trace_id: env.trace_id.clone(),
        ws_id: env.ws_id.clone(),
        error: RpcError {
            code: Some("request_invalid".to_string()),
            message: format!("invalid request: {e}"),
            detail: None,
            trace_id: Some(env.trace_id.clone()),
            ws_id: Some(env.ws_id.clone()),
        },
    })?;

    Ok(RpcInboundV1 {
        v,
        trace_id: env.trace_id,
        ws_id: env.ws_id,
        arming: env.arming,
        role: env.role,
        client: env.client,
        request: req,
    })
}

impl ProtocolReject {
    fn protocol_invalid() -> Self {
        protocol_invalid()
    }
}

pub async fn write_response_v1(
    writer: &mut OwnedWriteHalf,
    v: u8,
    trace_id: &str,
    ws_id: &str,
    resp: &Response,
) -> Result<()> {
    let env = ResponseEnvelopeV1::<Response> {
        v: v as u32,
        trace_id: trace_id.to_string(),
        ws_id: ws_id.to_string(),
        ok: true,
        response: Some(resp.clone()),
        error: None,
    };
    let payload = serde_json::to_string(&env).context("serialize rpc response envelope")?;
    writer.write_all(payload.as_bytes()).await?;
    writer.write_all(b"\n").await?;
    writer.flush().await?;
    Ok(())
}

pub async fn write_error_v1(
    writer: &mut OwnedWriteHalf,
    v: u8,
    trace_id: &str,
    ws_id: &str,
    err: &RpcError,
) -> Result<()> {
    let env = ResponseEnvelopeV1::<Response> {
        v: v as u32,
        trace_id: trace_id.to_string(),
        ws_id: ws_id.to_string(),
        ok: false,
        response: None,
        error: Some(err.clone()),
    };
    let payload = serde_json::to_string(&env).context("serialize rpc error envelope")?;
    writer.write_all(payload.as_bytes()).await?;
    writer.write_all(b"\n").await?;
    writer.flush().await?;
    Ok(())
}
