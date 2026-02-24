# Design Spine

This document is the single **source of navigation** for how YAI evolves from **law → design → delivery → proof → release** across repos.

It answers:
- Where does truth live?
- What do we write first?
- What must reference what?
- What closes the loop?

---

## The spine (L0 → L8)

### L0 — Constitution (normative truth)
**Lives in:** `yai-specs/` (authoritative) and pinned into other repos via `deps/yai-specs/`

- `contracts/*` (axioms, invariants, boundaries)
- `specs/*` (protocols, schemas, roles, errors)
- `formal/*` (TLA+ and proof obligations when present)
- `vectors/*` (test vectors; informative unless explicitly upgraded)

**This is the law.** Everything else is subordinate.  
If code or docs disagree with L0, **L0 wins**.

---

### L1 — Architecture model (human-readable)
**Lives in:** `docs/10-platform/architecture/*`

Explains the runtime model, boundaries, and component responsibilities.  
Architecture **must map to L0** (and never override it).

---

### L2 — RFCs (pre-decision design)
**Lives in:** `docs/20-program/21-rfc/*`

Used to explore alternatives, trade-offs, risks, and design space **before** freezing a decision.

---

### L3 — ADRs (frozen decisions)
**Lives in:** `docs/20-program/22-adr/*`

An ADR:
- chooses among alternatives
- states consequences and constraints
- references L0 as normative anchors (law/specs/contracts/formal)

ADRs are the “decision ledger”.

---

### L4 — Runbooks (execution plan)
**Lives in:** `docs/20-program/23-runbooks/*`

Runbooks convert ADR intent into a phased, verifiable sequence with acceptance gates.

Runbooks are the “how we execute” layer:
- phases (step-by-step)
- acceptance criteria
- mapping to Milestone Packs

---

### L5 — Milestone Packs (delivery packaging & audit unit)
**Lives in:** `docs/20-program/24-milestone-packs/*`

A Milestone Pack (MP) is the **unit of closure**:
- what changed
- which repos are impacted
- Type A / Type B classification (sync vs non-breaking)
- definition of done (DoD)
- evidence plan + pointers to evidence

**Rule:** a runbook phase maps to **one** MP (or a tightly-coupled micro-set).

---

### L6 — Issues (execution ledger, team coordination)
**Lives in:** GitHub Issues (per repo)

Issues are **work trackers**, not truth:
- each MP should have an Issue (or Epic Issue) as the canonical tracker
- Type B MPs require linked Issues across repos (paired execution)
- PRs close Issues; MPs define what “done” means

If you’re solo you *can* reduce Issues, but **never skip the MP** (that’s your audit artifact).

---

### L7 — Evidence & tests (proof-of-work)
**Lives in:** `docs/40-qualification/test-plans/*` + `tools/*` + CI artifacts/logs

Test plans define what “proved” means.  
Tools + CI runs provide repeatable evidence (ideally artifacted, non-skip for TRL claims).

---

### L8 — Proof Packs (public/internal credibility bundles)
**Lives in:** `docs/50-validation/proof/*` when published, `docs/50-validation/proof/.private/*` for local drafts

A Proof Pack bundles:
- evidence index
- non-skip gate outputs
- deterministic reject matrix (pos/neg) with trace_id + error codes
- requirements map (invariants/spec → tests/gates → artifacts)
- release notes (“what changed” + compat)
- demo script (2–4 min)
- storyline (onepager + deck outline)

**Pointer-only policy (recommended):**
- `yai-cli` and `yai-mind` contain a small pointer file to the canonical proof pack in `yai`
- `yai-specs` does **not** host proof packs (it hosts the contract)

---

## Traceability rules (non-negotiable)

Every artifact must point **up** and **down** where applicable:

### ADR must reference
- **Upstream:** L0 (`yai-specs/...`) via `law_refs`
- **Downstream:** 1+ runbooks (recommended; can be “TBD” early, but track it)

### Runbook must reference
- **Upstream:** 1+ ADRs (unless explicitly exempt: ops-only runbooks)
- **Downstream:** the list of MPs (phases)

### Milestone Pack must reference
- **Upstream:** runbook + phase
- **Lateral:** Issue ID(s) (and twin Issue IDs for Type B across repos)
- **Downstream:** evidence pointers (test plan refs + CI/gate artifact refs)

### Proof Pack must reference
- **Upstream:** the MP(s) it certifies
- **Downstream:** release tag(s) and any published demo assets

---

## Canonical “order of writing” (practical)

- If you are exploring: write an **RFC** (L2).
- If you are committing to a decision: write an **ADR** (L3).
- If you are implementing in phases: write/update a **Runbook** (L4).
- If you want to close a deliverable with evidence: write an **MP** (L5).
- If you want external/internal credibility: publish a **Proof Pack** (L8).

If in doubt: write an RFC first.  
If you are shipping: always close with an MP (even if the issue tracking is minimal).

---

## What closes the loop

A loop is closed only when:
- ADR intent is executed by a runbook phase,
- that phase is packaged as an MP with explicit DoD,
- evidence exists (tests/gates/artifacts) proving pos/neg behavior,
- (optionally) a Proof Pack bundles the evidence for sharing,
- (optionally) a release tag distributes an installable snapshot.

**Docs are plans. MPs are closure. Proof Packs are credibility. Releases are distribution.**
