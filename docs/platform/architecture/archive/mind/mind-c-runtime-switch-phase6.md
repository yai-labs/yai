# Mind C Runtime Switch — Phase 6

> Historical phase report: retained for migration traceability; active runtime guidance is in `mind/docs/README.md` and `mind/docs/mind-c-final-architecture.md`.

## Before this phase
Before phase 6, `mind/` had a working C implementation across lifecycle, memory, providers, transport, and cognition, but repository signals still looked mixed because Rust assets remained visible and unmanaged as legacy state.

## Runtime switch result
The operational default for `mind/` is now the C runtime:
- Primary build path: `make` in `mind/`.
- Primary daemon binary: `dist/bin/yai-mind` built from `src/main.c` + C subsystem modules.
- Primary test path: `make test` running `tests_c/*`.

Rust artifacts are retained for staged decommissioning only.

## C runtime flow coverage verified
The following baseline flows are validated on the C runtime:

1. Lifecycle flow
- `yai_mind_init`
- subsystem readiness flags in `yai_mind_runtime_state`
- `yai_mind_shutdown`

2. Provider flow
- provider registry init/lookup/default
- mock completion + embedding dispatch via client bridge

3. Memory flow
- graph node/edge operations
- domain operations
- memory query baseline

4. Cognition flow
- cognition request/session
- planner steps
- RAG context/prompt build with arena
- agent execution
- provider/memory usage
- cognition response

5. Transport flow
- protocol parse
- runtime dispatch
- formatted response
- one-shot UDS daemon smoke

## Runtime primary entrypoints
- `mind/Makefile` targets: `all`, `clean`, `check`, `test`, `run`.
- `src/main.c` is the daemon entrypoint for the standard operational path.

## Rust legacy declassification matrix
| Legacy Rust asset | Current role | Status | Removal phase |
|---|---|---|---|
| `Cargo.toml` | Legacy build metadata | `legacy-only` | Phase 7 |
| `build.rs` | Legacy cargo build script | `legacy-only` | Phase 7 |
| `src/main.rs` | Legacy executable entry | `legacy-only` | Phase 7 |
| `src/lib.rs` | Legacy crate surface | `legacy-only` | Phase 7 |
| `src/cognition/**/*.rs` | Historical cognition implementation | `legacy-only` | Phase 7 |
| `src/memory/**/*.rs` | Historical memory implementation | `legacy-only` | Phase 7 |
| `src/providers/**/*.rs` | Historical provider implementation | `legacy-only` | Phase 7 |
| `src/transport/**/*.rs` | Historical transport implementation | `legacy-only` | Phase 7 |
| `src/types/**/*.rs` | Historical type layer | `legacy-only` | Phase 7 |
| `src/workspace/**/*.rs` | Historical workspace layer | `legacy-only` | Phase 7 |
| `tests/**/*.rs` | Historical Rust tests | `legacy-only` | Phase 7 |

## Remaining gaps before purge phase
- C runtime is primary and runnable, but long-running UDS loop mode remains intentionally minimal.
- Cognition heuristics are baseline, not production-grade orchestration.
- Rust files are still present and must be removed in a dedicated destructive phase.

## Standard commands (C runtime)
```bash
cd mind
make clean
make -j4
make test
make run
```
