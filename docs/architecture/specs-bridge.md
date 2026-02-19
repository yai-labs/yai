---
id: ARCH-SPECS-BRIDGE
status: active
effective_date: 2026-02-19
revision: 1
owner: governance
law_refs:
  - deps/yai-specs/contracts/invariants/I-001-traceability.md
  - deps/yai-specs/specs/protocol/include/protocol.h
---

# Specs Bridge

## Role

Define mandatory citation and alignment rules between `yai` architecture docs and `deps/yai-specs` normative sources.

## Rules

- Every architecture document must include `law_refs` in frontmatter.
- Architecture statements that mention protocol/roles/errors must link a concrete `deps/yai-specs/...` path.
- Architecture docs must not duplicate normative spec text.
- If implementation diverges from target ADR intent, document as drift with explicit remediation target.

## Citation pattern

- Use repo-relative references only.
- Prefer concrete files over generic folders.

Examples:
- `deps/yai-specs/contracts/invariants/I-003-governance.md`
- `deps/yai-specs/contracts/boundaries/L1-kernel.md`
- `deps/yai-specs/specs/protocol/include/transport.h`

## Twin-change rule

If architecture claims require cross-repo behavior changes (`yai`, `yai-cli`, `yai-specs`), track as coordinated updates through runbook/MP references.
