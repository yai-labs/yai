use yai_mind::providers::client::{ProviderClient, ProviderRequest};
use yai_mind::transport::EngineClient;

#[test]
#[ignore = "requires running engine socket in local workspace"]
fn test_inference_v2_sovereign() {
    let ws_id = "test-ws";
    let client = EngineClient::connect(ws_id)
        .expect("engine must be active: ./bin/yai-engine <ws>");

    let req = ProviderRequest {
        provider: "E_RPC_INFERENCE".to_string(),
        model: "test".to_string(),
        payload: serde_json::json!({"prompt": "hello"}),
    };

    let response = client.call(req).expect("rpc call failed");
    assert!(response.payload.is_object());
}
