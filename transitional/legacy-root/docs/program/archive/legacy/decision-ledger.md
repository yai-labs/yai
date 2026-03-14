---
role: support
status: active
audience: governance
owner_domain: program
decision_id: DECISION-LEDGER
depends_on: [docs/program/README.md,docs/architecture/README.md]
supersedes: []
superseded_by: []
implements: []
evidenced_by: [docs/program/reports/runtime-convergence-report.md,docs/program/reports/audit-convergence-report.md]
related: [docs/program/adr/README.md,docs/program/rfc/README.md]
---
# Decision Ledger

# Purpose
Provide a single lifecycle map for active, pending, and superseded program decisions.

# Scope
Covers ADR/RFC/program evidence relationships for live governance docs.

# Relationships
- `docs/program/adr/README.md`
- `docs/program/rfc/README.md`
- `docs/program/reports/README.md`

# Canonical Role
Canonical lifecycle index for program decisions; source for decision state and supersession chains.

# Main Body

## Current Active Decisions
| id | owner_doc | status | supersedes | superseded_by | implements | evidenced_by |
| --- | --- | --- | --- | --- | --- | --- |
| ADR-031 | `docs/program/adr/adr-031-unified-graph-runtime.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-013 | `docs/program/adr/adr-013-acquisition-control.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-014 | `docs/program/adr/adr-014-secure-peering.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-015 | `docs/program/adr/adr-015-daemon-architecture.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-017 | `docs/program/adr/adr-017-edge-enforcement.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-019 | `docs/program/adr/adr-019-edge-binding-actions.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-022 | `docs/program/adr/adr-022-edge-policy-validity.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-023 | `docs/program/adr/adr-023-governed-sovereign-mesh.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-024 | `docs/program/adr/adr-024-mesh-discovery.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-025 | `docs/program/adr/adr-025-mesh-coordination.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-026 | `docs/program/adr/adr-026-sovereign-mesh-authority.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-028 | `docs/program/adr/adr-028-owner-remote-peer-ingress.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-016 | `docs/program/adr/adr-016-global-edge-policy-hierarchy.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-021 | `docs/program/adr/adr-021-workspace-policy-distribution.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-029 | `docs/program/adr/adr-029-overlay-integration.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-030 | `docs/program/adr/adr-030-source-edge-query-surfaces.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-032 | `docs/program/adr/adr-032-ai-grounding-case-state.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-004 | `docs/program/adr/adr-004-engine-execution.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-006 | `docs/program/adr/adr-006-unified-rpc.md` | accepted | [] | [] | [docs/program/rfc/rfc-002-unified-rpc-cli-contract.md] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-027 | `docs/program/adr/adr-027-secure-overlay-transport.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-001 | `docs/program/adr/adr-001-single-runtime.md` | accepted | [] | [] | [docs/program/rfc/rfc-001-runtime-topology-authority.md] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-003 | `docs/program/adr/adr-003-kernel-authority.md` | accepted | [] | [] | [docs/program/rfc/rfc-001-runtime-topology-authority.md] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-005 | `docs/program/adr/adr-005-mind-proposer.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-010 | `docs/program/adr/adr-010-boot-entrypoint.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-018 | `docs/program/adr/adr-018-runtime-observation.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-007 | `docs/program/adr/adr-007-workspace-isolation.md` | accepted | [] | [] | [docs/program/rfc/rfc-003-workspace-lifecycle-isolation.md] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-008 | `docs/program/adr/adr-008-connection-lifecycle.md` | accepted | [] | [] | [docs/program/rfc/rfc-003-workspace-lifecycle-isolation.md] | [docs/program/reports/runtime-convergence-report.md] |
| ADR-020 | `docs/program/adr/adr-020-workspace-authority-truth.md` | accepted | [] | [] | [] | [docs/program/reports/runtime-convergence-report.md] |
| RPT-002 | `docs/program/reports/audit-convergence-report.md` | active | [] | [] | [docs/program/adr/adr-012-audit-convergence-gates.md] | [] |
| RPT-001 | `docs/program/reports/runtime-convergence-report.md` | active | [] | [] | [docs/program/adr/adr-001-single-runtime.md,docs/program/adr/adr-003-kernel-authority.md] | [] |

## Superseded Chains
| id | owner_doc | status | supersedes | superseded_by | implements | evidenced_by |
| --- | --- | --- | --- | --- | --- | --- |
| ADR-002 | `docs/program/adr/adr-002-root-entrypoint.md` | historical | [] | [docs/program/adr/adr-001-single-runtime.md] | [] | [docs/program/reports/runtime-convergence-report.md] |

## Pending Draft Decisions
| id | owner_doc | status | intended_successor_to | notes |
| --- | --- | --- | --- | --- |
| ADR-011 | `docs/program/adr/adr-011-contract-runbook-lock.md` | draft | [] | Draft ADR pending ratification. |
| ADR-009 | `docs/program/adr/adr-009-engine-attachment.md` | draft | [] | Draft ADR pending ratification. |
| ADR-012 | `docs/program/adr/adr-012-audit-convergence-gates.md` | draft | [] | Draft ADR pending ratification. |
| RFC-004 | `docs/program/rfc/rfc-004-lock-pin-policy.md` | draft | [] | RFC proposal pending acceptance. |
| RFC-005 | `docs/program/rfc/rfc-005-formal-coverage-roadmap.md` | draft | [] | RFC proposal pending acceptance. |
| RFC-002 | `docs/program/rfc/rfc-002-unified-rpc-cli-contract.md` | draft | [] | RFC proposal pending acceptance. |
| RFC-001 | `docs/program/rfc/rfc-001-runtime-topology-authority.md` | draft | [] | RFC proposal pending acceptance. |
| RFC-003 | `docs/program/rfc/rfc-003-workspace-lifecycle-isolation.md` | draft | [] | RFC proposal pending acceptance. |

# Related Docs
- `docs/program/README.md`
- `docs/program/adr/README.md`
- `docs/program/rfc/README.md`
- `docs/program/reports/README.md`
