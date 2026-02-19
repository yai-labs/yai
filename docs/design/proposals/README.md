# Proposals

Proposals are pre-decision documents.
They are used when we need structured exploration before freezing an ADR.

## Use proposals when

- multiple options exist,
- risk/tradeoff analysis is still open,
- constraints are not yet fully pinned,
- decision quality benefits from explicit alternatives.

## Template

- `docs/templates/proposals/PRP-000-template.md`

## Current proposal set

- `docs/design/proposals/PRP-001-runtime-topology-and-authority.md`
  Runtime topology and authority boundaries (feeds ADR-001..ADR-005).
- `docs/design/proposals/PRP-002-unified-rpc-and-cli-contract.md`
  Unified RPC envelope and CLI contract alignment (feeds ADR-006, ADR-011).
- `docs/design/proposals/PRP-003-workspace-lifecycle-and-isolation.md`
  Workspace lifecycle and isolation guarantees (feeds ADR-007..ADR-010).
- `docs/design/proposals/PRP-004-contract-baseline-lock-and-pin-policy.md`
  Contract baseline lock and cross-repo pin policy (feeds ADR-011).
- `docs/design/proposals/PRP-005-formal-coverage-roadmap.md`
  Formal coverage roadmap for spec-critical domains (supports contract-proof alignment).

## Relationship with ADRs

- Proposal: exploration and option analysis.
- ADR: final decision and consequences.

Accepted proposals should be linked from the resulting ADR for historical traceability.
