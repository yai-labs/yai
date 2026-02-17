# YAI Documentation Policy

This policy defines the canonical rules for documentation in the YAI monorepo.
It is strict by design. Deviations must be explicit and justified.

---

## 1) Scope and Sources of Truth

1. `deps/yai-specs/contracts/` is the source of truth for:
   - formal specs (schemas, contracts, protocol headers)
   - invariants and boundaries
   - formal models (TLA+), compliance packs

2. `docs/` is the source of truth for:
   - operational runbooks (how to operate/build/test)
   - architecture narratives (how components fit together)
   - governance of documentation itself (this policy)
   - reference pointers that link to `deps/yai-specs/contracts/` (never duplicated specs)

3. Documentation MUST NOT duplicate formal specifications from `deps/yai-specs/contracts/`.
   `docs/` may only explain or point to `deps/yai-specs/contracts/`.

---

## 2) Canonical Document Types

YAI accepts these document types:

- **RUNBOOK (RB-*)**: operational procedures, step-by-step execution.
- **ADR (ADR-*)**: architecture decision records.
- **GUIDE (G-*)**: conceptual explanations and usage guidance.
- **REFERENCE (REF-*)**: pointers, indexes, quick links to `deps/yai-specs/contracts/` and other docs.

A document MUST fit exactly one type.

---

## 3) Naming Canon (No Versions in Filenames)

1. Filenames MUST be stable identifiers. Filenames MUST NOT contain versions.
   Examples:
   - ✅ `RB-ROOT-HARDENING.md`
   - ✅ `ADR-ENGINE-ATTACHMENT.md`
   - ❌ `RB-ROOT-HARDENING-v2.md`
   - ❌ `data-plane-v5.md`

2. Versions are metadata inside the document, not in the filename.

3. Each document ID is the filename without extension.
   Example: `RB-ROOT-HARDENING.md` → `id: RB-ROOT-HARDENING`

---

## 4) Mandatory Metadata Header

Every document in `docs/` MUST begin with a metadata block:

```yaml
---
id: <STABLE_ID>
status: draft|active|deprecated
effective_date: YYYY-MM-DD
revision: <integer>
supersedes: [<id>, ...]
owner: <area>
law_refs:
  - <path in deps/yai-specs/contracts/...>
tags: [<tag>, ...]
---
```

Rules:

- `id` MUST match the filename.
- `revision` increments by 1 on meaningful changes.
- `supersedes` is used when replacing a document.
- `law_refs` MUST list the controlling `deps/yai-specs/contracts/` references when applicable.

---

## 5) Folder Canon (docs/)

`docs/` MUST follow this structure:

- `docs/architecture/`
  Architecture overviews and ADR index.
- `docs/architecture/adr/`
  ADR documents only.
- `docs/governance/`
  Documentation governance and policy.
- `docs/operations/`
  Operational material.
- `docs/operations/runbooks/`
  RB documents only.
- `docs/operations/tests/`
  Test procedures and test matrices (human-run).
- `docs/guides/`
  Guides and conceptual docs.
- `docs/reference/`
  Indexes, pointers, and curated links (may reference `deps/yai-specs/contracts/`).

Prohibited:

- `docs/editorial/` (subjective naming; not allowed)
- `deps/yai-specs/` (canonical specs live here)

---

## 6) Content Rules (Runbooks)

A Runbook MUST contain, in this order:

1. Purpose (one paragraph)
2. Preconditions (explicit, testable)
3. Inputs (flags, environment variables, paths)
4. Procedure (numbered steps, executable commands)
5. Verification (how to confirm success)
6. Failure Modes (deterministic symptoms + fixes)
7. Rollback (how to return to safe state)
8. References (`law_refs` + internal links)

Runbooks MUST be executable without interpretation.
If a step requires judgment, the rule is incomplete and MUST be tightened.

---

## 7) Content Rules (ADRs)

An ADR MUST contain, in this order:

1. Context
2. Decision
3. Rationale
4. Alternatives Considered
5. Consequences
6. Law Alignment (explicit references)
7. Status (draft/accepted/deprecated)

ADRs MUST NOT contain operational procedures. Those belong in Runbooks.

---

## 8) Deprecation and Replacement

1. Deprecation is explicit:
   - `status: deprecated`
   - Add a banner line at the top:
     `DEPRECATED — replaced by <ID>`

2. Deprecated documents are not deleted unless legally required.
   If moved, they go to:
   - `docs/reference/archive/`

3. Replacement MUST be declared via `supersedes`.

---

## 9) Link and Reference Discipline

1. Links MUST be relative paths inside the repo.
2. Documents MUST prefer pointing to `deps/yai-specs/contracts/` rather than copying content.
3. If a doc references a protocol field, it MUST link to the canonical header in `deps/yai-specs/`.

---

## 10) Review and Merge Rules

1. Any change to:
   - `docs/governance/`
   - `docs/architecture/adr/`
   - `deps/yai-specs/contracts/`
   
   requires a review before merge (no self-merge).

2. Runbooks that affect runtime safety (L0/L1/L2 operations) require:
   - verification section present
   - rollback section present

---

## 11) Minimal Templates (Canonical)

New documents MUST be created from templates located in:

- `docs/templates/`

Templates are mandatory to ensure uniform structure.

---

## Enforcement

If a document violates this policy, it is non-canonical and MUST be fixed before it becomes `active`.

---

# Template: RUNBOOK

```markdown
---
id: RB-<NAME>
status: draft
effective_date: YYYY-MM-DD
revision: 1
supersedes: []
owner: operations
law_refs: []
tags: []
---

# RB-<NAME>

## Purpose
<one paragraph>

## Preconditions
- [ ] <precondition>

## Inputs
- <flags/env/paths>

## Procedure
1. <step>

## Verification
- <how to confirm>

## Failure Modes
- Symptom: <...>
  Fix: <...>

## Rollback
- <rollback steps>

## References
- <links>
```

---

# Template: ADR

```markdown
---
id: ADR-<NAME>
status: draft
effective_date: YYYY-MM-DD
revision: 1
supersedes: []
owner: architecture
law_refs: []
tags: []
---

# ADR-<NAME>

## Context
<...>

## Decision
<...>

## Rationale
<...>

## Alternatives Considered
- <...>

## Consequences
<...>

## Law Alignment
- <law refs>

## Status
draft|accepted|deprecated
```
