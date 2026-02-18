# Design Spine

This document is the single “source of navigation” for how YAI evolves from law → design → delivery.

It answers:
- Where does truth live?
- What do we write first?
- What must reference what?
- What closes the loop?

## The spine (L0 → L7)

### L0 — Constitution (normative truth)
**Lives in:** `deps/yai-specs/`
- `contracts/*` (axioms, invariants, boundaries)
- `specs/*` (protocols, schemas, roles, errors)
- `formal/*` (TLA+ and proof obligations when present)

This is the law. Everything else is subordinate.

### L1 — Architecture model (human-readable)
**Lives in:** `docs/architecture/*`

Explains the system and maps concepts to specs.  
Architecture does not override L0.

### L2 — Proposals (pre-decision, optional)
**Lives in:** `docs/design/proposals/*`

Use proposals to explore alternatives, trade-offs, risks, and design space **before** freezing a decision.

### L3 — ADRs (frozen decisions)
**Lives in:** `docs/design/adr/*`

An ADR:
- chooses among alternatives
- states consequences
- points to L0 (contracts/invariants/boundaries/specs) as normative anchors

### L4 — Runbooks (execution plan)
**Lives in:** `docs/runbooks/*`

Runbooks convert ADR intent into a phased, verifiable sequence with acceptance gates.

### L5 — Milestone Packs (delivery packaging & audit)
**Lives in:** `docs/milestone-packs/*`

MPs close outcomes:
- what changed
- what evidence proves it
- what repos are impacted
- what compatibility classification applies

### L6 — Evidence & tests (proof-of-work)
**Lives in:** `docs/test-plans/*` + `tools/*` + CI logs

Test plans define what “proved” means.  
Tools and CI runs provide repeatable evidence.

### L7 — Release (distribution freeze)
Tag/release is a distribution event, not required for every MP.

---

## Traceability rules (non-negotiable)

Every artifact must point “up” and “down” where applicable:

- **ADR** must reference:
  - upstream: L0 (`deps/yai-specs/...`) via `law_refs`
  - downstream: 1+ runbooks (optional list at first, but recommended)

- **Runbook** must reference:
  - upstream: 1+ ADRs (unless explicitly exempt: operations-only)
  - downstream: MPs (phases)

- **Milestone Pack** must reference:
  - upstream: runbook + phase
  - lateral: linked Issue-ID(s) when applicable
  - downstream: evidence pointers (commands run, CI logs, fixtures)

---

## When to write what (practical)

- If you are exploring: write a **Proposal** (L2).
- If you are committing to a decision: write an **ADR** (L3).
- If you are implementing in phases: write/update a **Runbook** (L4).
- If you want to close a deliverable with evidence: write an **MP** (L5).

If in doubt: write a Proposal first.
