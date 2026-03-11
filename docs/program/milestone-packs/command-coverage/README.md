---
id: MP-COMMAND-COVERAGE-INDEX
status: active
runbook: N/A
phase: index
adrs:
  - docs/program/adr/ADR-006-unified-rpc.md
  - docs/program/adr/ADR-012-audit-convergence-gates.md
spec_anchors:
  - ../law/registry/commands.v1.json
issues:
  - "N/A: command-coverage index"
---

# Command Coverage Milestone Packs

Purpose:
- Track broad registry command-surface expansion and implementation waves that are not strictly WS-lifecycle scoped.

Relation to WS runbook:
- `docs/program/milestone-packs/runtime-baselines/workspaces-lifecycle.md` defines WS semantics/invariants.
- This track defines coverage work across non-WS groups.

Command coverage sequence:
- `docs/program/milestone-packs/command-coverage/MP-COMMAND-COVERAGE-0.2.1.md` (root+kernel+boot status wave)
- `docs/program/milestone-packs/command-coverage/MP-COMMAND-COVERAGE-0.2.2.md` (root+kernel+boot operational wave)
- `docs/program/milestone-packs/command-coverage/MP-COMMAND-COVERAGE-0.2.3.md` (mind)
- `docs/program/milestone-packs/command-coverage/MP-COMMAND-COVERAGE-0.2.4.md` (orch)
- `docs/program/milestone-packs/command-coverage/MP-COMMAND-COVERAGE-0.2.5.md` (substrate)

Notes:
- These packs were originally planned under workspaces-lifecycle and then re-scoped.
