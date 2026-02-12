# YAI

YAI is a layered runtime system with a deterministic control plane and an operator-first CLI.

Core goals:
- deterministic runtime lifecycle (`up/down/status/events`)
- law-driven boundaries and verifiable contracts
- graph-centric memory and awareness hooks
- provider pairing/trust workflow for LLM integration

## Architecture

YAI is organized in layers:
- `law/` (L0): axioms, invariants, specs, formal models
- `kernel/` (L1): low-level authority/enforcement runtime in C
- `engine/` (L2): execution bridge and runtime services in C
- `mind/` (L3): control plane, graph memory, providers, CLI in Rust
- `scripts/` (L5): canonical verify/gate/suite runners

Key docs:
- `docs/STRATIFICATION.md`
- `docs/RUNBOOKS.md`
- `docs/DATASETS.md`
- `law/specs/cli/CLI_PUBLIC_INTERFACE.md`
- `law/specs/cli/TUI_COCKPIT_V1.md` (deprecated, historical only)

## Quick Start

### 1) Install `yai`

```bash
cd /Users/francescomaiomascio/Developer/YAI/yai/mind
cargo install --path . --locked --force --bin yai
```

If `yai` is not found:

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
hash -r
```

### 2) Start a workspace

```bash
cd /Users/francescomaiomascio/Developer/YAI/yai
yai up --ws dev --build --detach
yai status --ws dev --json
```

### 3) Open live monitor

```bash
yai monitor --ws dev
```

For graphical UX, use YX (`yai-yx`) as the canonical GUI client of the same control socket.

```bash
# in /Users/francescomaiomascio/Developer/YAI/yai-yx
YX_MODE=auto YX_SOCK="$HOME/.yai/run/dev/control.sock" make dev
```

## Provider Pairing (LAN)

Check your provider endpoint first:

```bash
curl -sS http://<LAN_IP>:8080/v1/models
```

Pair + trust + attach:

```bash
yai providers --ws dev pair \
  "remote:http://<LAN_IP>:8080/v1/chat/completions" \
  "http://<LAN_IP>:8080/v1/chat/completions" \
  "<MODEL_NAME>"

yai providers trust --id "remote:http://<LAN_IP>:8080/v1/chat/completions" --state trusted
yai providers --ws dev attach "remote:http://<LAN_IP>:8080/v1/chat/completions"
yai providers --ws dev status
```

## Verification and Gates

Canonical runners:
- `scripts/yai-verify <name>`
- `scripts/yai-gate <name> [args...]`
- `scripts/yai-suite <path> [args...]`

Examples:

```bash
scripts/yai-verify core
scripts/yai-gate ws dev
scripts/yai-gate graph dev
scripts/yai-suite levels/l0-l7
scripts/yai-suite ops/no-llm-360
```

Direct suite:

```bash
DATASET_GATE=1 WS_PREFIX=ops360 ./scripts/suites/levels/l0-l7.sh
```

## Build

Core C runtime:

```bash
make all
```

Mind (Rust):

```bash
cd mind
cargo build --release
```

## Repository Map

- `datasets/`: canonical datasets (data-first, not runtime orchestration)
- `Data/`: local runtime DB/artifacts (production setups usually exclude it from VCS)
- `docs/specs/`: editorial index and pointers
- `law/specs/`: canonical machine/contract specs
- `mind/src/cli/`: CLI thin client commands and runtime config paths
- `mind/src/control/`: daemon/control-plane services
- `mind/src/runtime/`: planning/scheduling/runtime core
- `mind/src/transport/`: rpc + bridge boundaries
- `mind/src/cognition/`: agents/llm/memory/rag stack
- `scripts/gates/`, `scripts/verify/`, `scripts/suites/`: deterministic pipeline

## Troubleshooting

Provider appears attached but chat fails:
- verify endpoint reachability with `curl`
- ensure attached provider is not revoked
- check `yai providers --ws <ws> status`

CLI command not found:
- check `which yai`
- ensure `~/.cargo/bin` in PATH

Runtime issues:

```bash
yai down --ws dev --force
yai up --ws dev --build --detach
yai status --ws dev --json
```
