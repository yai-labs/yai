# Traceability Map

This is the navigable index that prevents docs drift.
It maps proposal and decision artifacts to contract anchors and delivery evidence.

## How to use this map
- Add a row when a new Proposal, ADR, or Runbook is introduced.
- Keep links as repo-relative paths.
- Anchor every row to real `deps/yai-specs` paths.

## Proposal to ADR map

| Proposal (L2) | Spec anchors (L0) | Target ADRs (L3) |
|---|---|---|
| `docs/design/proposals/PRP-001-runtime-topology-and-authority.md` | `deps/yai-specs/contracts/axioms/A-002-authority.md`<br/>`deps/yai-specs/contracts/boundaries/L1-kernel.md`<br/>`deps/yai-specs/contracts/boundaries/L2-engine.md` | `docs/design/adr/ADR-001-single-runtime.md`<br/>`docs/design/adr/ADR-002-root-entrypoint.md`<br/>`docs/design/adr/ADR-003-kernel-authority.md`<br/>`docs/design/adr/ADR-004-engine-execution.md`<br/>`docs/design/adr/ADR-005-mind-proposer.md` |
| `docs/design/proposals/PRP-002-unified-rpc-and-cli-contract.md` | `deps/yai-specs/specs/protocol/include/transport.h`<br/>`deps/yai-specs/specs/protocol/include/protocol.h`<br/>`deps/yai-specs/specs/protocol/runtime/include/rpc_runtime.h`<br/>`deps/yai-specs/specs/cli/schema/commands.v1.json` | `docs/design/adr/ADR-006-unified-rpc.md`<br/>`docs/design/adr/ADR-011-contract-baseline-lock.md` |
| `docs/design/proposals/PRP-003-workspace-lifecycle-and-isolation.md` | `deps/yai-specs/contracts/boundaries/L1-kernel.md`<br/>`deps/yai-specs/specs/protocol/include/session.h`<br/>`deps/yai-specs/contracts/invariants/I-002-determinism.md`<br/>`deps/yai-specs/contracts/invariants/I-006-external-effect-boundary.md` | `docs/design/adr/ADR-007-workspace-isolation.md`<br/>`docs/design/adr/ADR-008-connection-lifecycle.md`<br/>`docs/design/adr/ADR-009-engine-attachment.md`<br/>`docs/design/adr/ADR-010-boot-entrypoint.md` |
| `docs/design/proposals/PRP-004-contract-baseline-lock-and-pin-policy.md` | `deps/yai-specs/formal/traceability.v1.json`<br/>`deps/yai-specs/formal/spec_map.md`<br/>`deps/yai-specs/contracts/invariants/I-001-traceability.md`<br/>`deps/yai-specs/contracts/invariants/I-007-compliance-context-required.md` | `docs/design/adr/ADR-011-contract-baseline-lock.md` |
| `docs/design/proposals/PRP-005-formal-coverage-roadmap.md` | `deps/yai-specs/formal/spec_map.md`<br/>`deps/yai-specs/formal/tla/YAI_KERNEL.tla`<br/>`deps/yai-specs/formal/bindings/BINDING_PROTOCOL.md`<br/>`deps/yai-specs/formal/bindings/BINDING_CLI.md` | `docs/design/adr/ADR-006-unified-rpc.md`<br/>`docs/design/adr/ADR-011-contract-baseline-lock.md` |

## ADR to delivery map

| Capability / Track | ADR (L3) | Runbook (L4) | MP (L5) | Tests / Evidence (L6) |
|---|---|---|---|---|
| Root hardening | `docs/design/adr/ADR-006-unified-rpc.md` | `docs/runbooks/root-hardening.md` | `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md` | `docs/test-plans/*` + CI logs + runbook commands |
| Workspaces lifecycle | `docs/design/adr/ADR-007-workspace-isolation.md` | `docs/runbooks/workspaces-lifecycle.md` | *(TBD)* | *(TBD)* |

Notes:
- Keep this map synchronized whenever proposal scope or ADR targets change.
- Do not invent new anchors: always anchor to `deps/yai-specs` paths.
