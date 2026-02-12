# YAI

YAI is a layered runtime system with a deterministic control plane and an operator-first CLI/TUI.

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
- `mind/` (L3): control plane, graph memory, providers, CLI/TUI in Rust
- `scripts/` (L5): canonical verify/gate/suite runners

Key docs:
- `docs/STRATIFICATION.md`
- `docs/RUNBOOKS.md`
- `docs/DATASETS.md`
- `law/specs/cli/CLI_PUBLIC_INTERFACE.md`
- `law/specs/cli/TUI_COCKPIT_V1.md`

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

Runtime issues:

```bash
yai down --ws dev --force
yai up --ws dev --build --detach
yai status --ws dev --json
```
