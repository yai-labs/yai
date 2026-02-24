# Test Providers (R4)

## Quick Gate
```bash
cd "$(git rev-parse --show-toplevel)"
tools/bin/yai-gate providers dev
```

## Trust Override (unblock L7)
```bash
export WS=dev
export PROVIDER_ID='remote:http://127.0.0.1:18080/v1/chat/completions?ws=dev'

yai providers trust --id "$PROVIDER_ID" --state trusted
```

## Gate Modes
Non-strict (default):
```bash
tools/bin/yai-gate providers "$WS"
```
If no trusted provider is available, it exits `0` with:
`SKIP: no trusted provider (non-strict)`

Strict:
```bash
REQUIRE_ACTIVE_PROVIDER=1 tools/bin/yai-gate providers "$WS"
```
Fails if no trusted provider exists.

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

## Mode Test
```bash
tools/bin/yai-gate providers-modes-test
```
