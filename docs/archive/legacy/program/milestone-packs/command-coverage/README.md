---
id: MP-COMMAND-COVERAGE-INDEX
status: active
runbook: N/A
phase: index
adrs:
  - docs/program/adr/adr-protocol-006-unified-rpc.md
  - docs/program/adr/adr-program-012-audit-convergence-gates.md
spec_anchors:
  - ../governance/registry/commands.v1.json
issues:
  - "N/A: command-coverage index"
---

# Command Coverage Milestone Packs

Purpose:
- Track broad registry command-surface expansion and implementation waves that are not strictly WS-lifecycle scoped.

Relation to WS runbook:
- `docs/archive/legacy/program/milestone-packs/runtime-baselines/workspace/mp-runtime-000-workspaces-lifecycle.md` defines WS semantics/invariants.
- This track defines coverage work across non-WS groups.

Command coverage sequence:
- `docs/archive/legacy/program/milestone-packs/command-coverage/mp-runtime-000-command-coverage-v0-2-5.md` (root+kernel+boot status wave)
- `docs/archive/legacy/program/milestone-packs/command-coverage/mp-runtime-000-command-coverage-v0-2-5.md` (root+kernel+boot operational wave)
- `docs/archive/legacy/program/milestone-packs/command-coverage/mp-runtime-000-command-coverage-v0-2-5.md` (mind)
- `docs/archive/legacy/program/milestone-packs/command-coverage/mp-runtime-000-command-coverage-v0-2-5.md` (orch)
- `docs/archive/legacy/program/milestone-packs/command-coverage/mp-runtime-000-command-coverage-v0-2-5.md` (substrate)

Notes:
- These packs were originally planned under workspaces-lifecycle and then re-scoped.


Retention rule: latest-only (intermediate versions are evicted in C17.5).
