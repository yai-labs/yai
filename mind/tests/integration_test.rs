use mind::transport::{EngineClient, YaiCommand};
use serde_json::Value;

#[test]
fn test_inference_v2_sovereign() {
    // 1. Connessione al Workspace (Assicurati che l'Engine sia attivo con questo WS)
    let ws_id = "test-ws";
    let mut client = EngineClient::connect(ws_id)
        .expect("L'Engine deve essere attivo! Esegui: ./bin/yai-engine test-ws");

    println!("[TEST] Connesso al socket dell'Engine.");

    // 2. Invio comando di Inference
    let prompt = "Spiega la legge della termodinamica come se fossi un AI sovrana.";
    let response = client
        .call_inference(prompt)
        .expect("Errore durante la chiamata RPC");

    // 3. Verifica Risposta
    println!("[TEST] Risposta ricevuta: {}", response);

    assert_eq!(response["status"], "success");
    assert!(response["content"].as_str().is_some());

    println!("[TEST] Blocco 1 Verificato: Comunicazione binaria L3->L2 completata.");
}
