# Architecture Stratification

This file is the operational contract for YAI strata.
It defines allowed responsibilities, coupling boundaries, invariants, and
certifying test gates.

## Strata Contract

| Layer | Responsibility | API/ABI Surface | Storage | Events | Certification Gates |
|---|---|---|---|---|---|
| L0 | Law specs + canonical ABI + identifiers | `law/specs/vault/*`, `law/specs/protocol/*`, generated headers | ABI contracts, schema artifacts | N/A (spec level) | `scripts/verify/law-kernel.sh`, `scripts/verify/core.sh` |
| L1 | Kernel authority, enforcement, state transitions, transport/session guards | `kernel/include/*`, `yai_vault_t`, kernel FSM | runtime vault/shared memory surfaces | state/authority transitions, violations | `scripts/verify/law-kernel.sh`, `scripts/gates/ws.sh` |
| L2 | Deterministic engine execution + bridge + cortex + storage proxy | `engine/include/*`, engine runtime interfaces | engine runtime state, deterministic outputs | execution/scale evidence | `scripts/gates/cortex.sh`, `scripts/verify/core.sh` |
| L3 | Mind control-plane, providers, memory graphs, rpc transport | `mind/src/control/*`, `mind/src/transport/rpc/*`, `mind/src/cognition/memory/*` | working/episodic/semantic/vector stores | process + provider + graph/memory events | `scripts/gates/events.sh`, `scripts/gates/graph.sh`, `scripts/gates/providers.sh` |
| L4 | Public CLI product interface | `mind/src/cli/commands/*`, `yai` CLI | CLI state wrappers over run dir | command-facing status/events | `yai verify core`, `yai verify full`, `yai test smoke` |
| L5 | Deterministic release/test pipeline | `scripts/verify/*`, `scripts/gates/*`, `scripts/suites/*` | test artifacts, run logs | gate pass/fail evidence | `scripts/suites/levels/l0-l7.sh`, `scripts/suites/ops/no-llm-360.sh` |

## Compute Rule

- Probabilistic compute lives in **L3**.
- Deterministic commit is enforced through **L1/L2/L3 gates**.
- No probabilistic path can bypass deterministic authority and commit surfaces.

## Memory Subgraphs Contract

Subgraphs:
- `working`
- `episodic`
- `semantic`

Allowed directional coupling:
- `working -> episodic` (event materialization)
- `working -> semantic` (explicit graph operations)
- `episodic -> semantic` (derived indexing/projection, no authority escalation)

Disallowed:
- `semantic -> L1 authority mutation`
- `episodic -> direct kernel authority mutation`
- any bypass that turns probabilistic inference into direct commit outside gates

## Allowed Edge List (Operational)

Current explicit relations used by gates/runtime include:
- `blocked_by_kernel`
- `trusted_by`
- `attached_to`
- `has_capability`

Any new relation must be:
1. explicitly introduced in code/specs,
2. covered by at least one gate/suite assertion.

## Layer Coupling Rules

- L(n) may depend on L(n-1) contracts, never reinterpret them.
- L4 must not bypass L1/L2/L3 enforcement surfaces.
- L5 must remain executable from a clean environment with deterministic pass/fail outputs.

## Certification Sequence

1. `scripts/suites/levels/l0-l7.sh`
2. `scripts/suites/ops/no-llm-360.sh`
3. Prompt/LLM suites (separate phase)
