# Test Providers (R4)

## Quick Gate
```bash
cd ~/Developer/YAI/yai
./scripts/gate-providers.sh dev
```

## Manual Flow
```bash
export WS=dev
export PROVIDER_ID='remote:http://127.0.0.1:18080/v1/chat/completions'

yai providers --ws "$WS" discover
yai providers --ws "$WS" pair "$PROVIDER_ID" "http://127.0.0.1:18080/v1/chat/completions" "qwen-test"
yai providers --ws "$WS" attach "$PROVIDER_ID"
yai providers --ws "$WS" status
yai providers --ws "$WS" detach
yai providers --ws "$WS" revoke "$PROVIDER_ID"
```

## Expected
- `~/.yai/trust/providers.json` exists with `version=1`
- provider has `trust_state=revoked` after revoke
- `events.log` includes:
  - `provider_discovered`
  - `provider_paired`
  - `provider_attached`
  - `provider_detached`
  - `provider_revoked`
- attach after revoke is rejected
