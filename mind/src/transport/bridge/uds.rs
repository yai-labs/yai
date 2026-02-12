// src/bridge/uds.rs
#![allow(dead_code)]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

pub struct UdsConnector {
    path: String,
}

impl UdsConnector {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    /// Invia un comando all'Engine tramite il socket Unix
    pub fn send_command(&self, _cmd: &str) -> tokio::io::Result<String> {
        // Implementiamo una logica asincrona dentro un blocco bloccante se necessario,
        // o lo gestiamo direttamente nel server.rs
        unimplemented!("Verrà invocato direttamente dal server asincrono")
    }

    pub async fn call_engine(&self, payload: &[u8]) -> tokio::io::Result<Vec<u8>> {
        let mut stream = UnixStream::connect(&self.path).await?;

        // Scrittura comando
        stream.write_all(payload).await?;
        stream.shutdown().await?;

        // Lettura risposta
        let mut response = Vec::new();
        stream.read_to_end(&mut response).await?;

        Ok(response)
    }
}
