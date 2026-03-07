# Mind C Providers + Transport — Phase 4

## Objective
Phase 4 ports the runtime connectivity layer from Rust to native C:
- provider/runtime-ready registry
- client bridge
- embedder mock baseline
- protocol boundary
- UDS server baseline
- daemon wiring for one request end-to-end

## Rust -> C mapping

### Providers
| Rust source | C target | Status |
|---|---|---|
| `src/providers/registry.rs` | `src/providers/provider_registry.c` | implemented runtime-ready baseline |
| `src/providers/types.rs` | `include/mind_providers.h`, `include/mind_types.h` | consolidated |
| `src/providers/client.rs` | `src/providers/client_bridge.c` | implemented |
| `src/providers/embedders/base.rs` | `src/providers/embedder_mock.c` (baseline utility) | implemented baseline |
| `src/providers/embedders/mock.rs` | `src/providers/mock_provider.c` | implemented |

### Transport
| Rust source | C target | Status |
|---|---|---|
| `src/transport/protocol.rs` | `src/transport/protocol.c` + `include/mind_transport.h` | implemented |
| `src/transport/uds_server.rs` | `src/transport/uds_server.c` | implemented baseline (single request) |
| `src/transport/mod.rs` | `src/transport/transport.c` | implemented |

## Provider architecture
- Registry supports:
  - register provider
  - select default
  - lookup by name
  - orderly destroy
- Provider API is vtable-based:
  - `completion`
  - `embedding`
  - `destroy`
- Client bridge:
  - request construction
  - provider dispatch via registry
  - response/error normalization

## Embedder baseline
- `embedder_mock.c` produces deterministic vectors from input text.
- `mock_provider.c` delegates embedding to `yai_mind_embedder_mock_fill`.

## Protocol boundary
Text protocol (line based):
- `PING`
- `COMPLETE <payload>`
- `EMBED <payload>`
- `QUERY <payload>`

Protocol pipeline:
1. parse raw request -> typed request struct
2. dispatch -> provider/memory handlers
3. build typed response
4. format response line: `STATUS <http-like> CODE <mind-code> <payload>`

## UDS server baseline
`yai_mind_uds_server_run_once(path)`:
- bind/listen on UDS path
- accept one connection
- read one request
- dispatch via protocol handler
- write one response
- close and unlink socket

Single-request mode is deliberate for predictable smoke tests.

## Daemon integration
`main.c` now supports:
- `--check` (lifecycle smoke)
- `--serve-once [--socket <path>]`

Flow in daemon:
- init lifecycle
- seed one semantic graph node for query flow
- serve one UDS request (if requested)
- shutdown lifecycle

## End-to-end flow implemented
Example request:
- `COMPLETE daemon smoke`

Flow:
- UDS receives raw line
- protocol parses into `COMPLETE`
- dispatch calls provider client bridge
- client bridge calls mock provider via registry
- response is formatted and written back on UDS

## Tests added
- `tests_c/test_providers.c`
  - registry lookup/default, completion, embedding
- `tests_c/test_transport.c`
  - protocol + dispatch via raw handler (PING/COMPLETE/QUERY)
- `tests_c/test_mind_daemon_smoke.c`
  - child daemon `--serve-once`
  - client UDS request/response validation

## Deferred/stubbed items
- multi-request/event-loop UDS runtime
- production serialization protocol
- real networked/RPC provider integrations
- cognition request orchestration integration

## Residual risks for next phase
- cognition path still not connected to provider/transport intents.
- protocol is intentionally simple; needs versioning strategy later.
- UDS server currently single-request baseline, not long-running service loop.
