---
id: RB-SPECS-REFACTOR-FOUNDATION
title: Specs Refactor Foundation
status: draft
owner: governance
effective_date: 2026-02-19
revision: 3
supersedes: []
depends_on:
  - RB-CONTRACT-BASELINE-LOCK
adr_refs:
  - docs/20-governance/22-adr/ADR-011-contract-baseline-lock.md
  - docs/20-governance/22-adr/ADR-012-audit-convergence-gates.md
decisions:
  - docs/20-governance/22-adr/ADR-011-contract-baseline-lock.md
  - docs/20-governance/22-adr/ADR-012-audit-convergence-gates.md
related:
  adr:
    - docs/20-governance/22-adr/ADR-011-contract-baseline-lock.md
    - docs/20-governance/22-adr/ADR-012-audit-convergence-gates.md
  specs:
    - deps/yai-specs/SPEC_MAP.md
    - deps/yai-specs/REGISTRY.md
    - deps/yai-specs/VERSIONING.md
    - deps/yai-specs/COMPATIBILITY.md
  test_plans: []
  tools:
    - tools/bin/yai-docs-trace-check
    - tools/bin/yai-proof-check
    - tools/release/check_pins.sh
---

# RB-SPECS-REFACTOR-FOUNDATION - Pre-Hardening Specs Program

## 0) Intent
This runbook establishes a deterministic, enterprise-grade foundation for **`yai-specs`** and its consumers (**`yai`**, **`yai-cli`**) *before* runtime hardening tracks (Root Hardening, Workspace Lifecycle, Engine Attach, Data Plane).

It turns the "specs refactor" from a vague refactor into **phased, gated work** with:
- strict *mapping-only* steps first (no semantic drift),
- explicit *consumer wiring* steps,
- CI guardrails (non-mergable regressions),
- toolchain & policy gates,
- formal traceability (contracts <-> specs <-> vectors <-> formal),
- TLA model-check quick/deep pipeline.

---

## 1) Definitions (the layering you were missing)
### Runbook
A **program of work** composed of **phases** with **exit gates**. It is the "operational book" that coordinates work across repos.

### Phase (0.1.X)
A **closure unit** of the runbook with:
- a single *claim* ("what becomes true"),
- bounded scope,
- defined *gate* (commands + required CI checks),
- a **Milestone Pack (MP)** that captures closure evidence.

### Milestone Pack (MP)
A *closure artifact* for a phase: what changed, proof evidence, links to issues/PRs, and the final gate output.

### Issues and PRs
- **Issue** = operational task unit (work item), may create 1+ PRs.
- **PR** = code/doc change unit that merges to the canonical branch.

> Rule: a "refactor" runbook can span many PRs. That's normal. The *validation* happens per phase gate.

---

## 2) Preconditions (hard rules)
- [ ] `RB-CONTRACT-BASELINE-LOCK` has been executed or at least stabilized (ADR-011 baseline available).
- [ ] Cross-repo pin governance is active and runnable (e.g. `tools/release/check_pins.sh` in the consumer).
- [ ] No direct development is performed inside consumer vendored specs trees (e.g. `yai/deps/yai-specs`).
- [ ] Any change in `yai-specs` that affects consumers is coordinated via pins/tags and verified in consumers.

---

## 3) Inputs / Repos
### Repos involved
- `yai-specs` (canonical specs)
- `yai` (consumer/runtime)
- `yai-cli` (consumer)

### Canonical references (in `yai-specs`)
- `SPEC_MAP.md`, `REGISTRY.md`, `VERSIONING.md`, `COMPATIBILITY.md`
- `contracts/**`, `specs/**`, `vectors/**`, `formal/**`, `compliance/**`

### Consumer checks (examples)
- `yai` build + verify scripts
- `yai-cli` build + tests/vectors

### 3.1 Audit Convergence Binding (Wave 1)
This runbook is Wave 1 under:
- `docs/30-program/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
- `docs/30-program/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`

Claims source of truth:
- `docs/60-validation/audits/claims/infra-grammar.v0.1.json`

Wave tracking:
- `https://github.com/yai-labs/yai/issues/142`
- `https://github.com/yai-labs/yai-specs/issues/9`

Mandatory closure policy:
- for mandatory evidence checks, `SKIP` is treated as `FAIL`.

---

## 4) Sequencing (global program position)
1. Contract baseline lock runbook (first)
2. **Specs refactor foundation runbook** (this document)
3. Root hardening runbook
4. Workspace lifecycle
5. Engine attach
6. Data plane

> You can run multiple runbooks in parallel **only** if they do not collide in scope (same artifacts / same enforcement boundary). For specs foundation: treat it as "upstream" and keep it clean.

---

## 5) Working Model (how you execute without chaos)
### Repo-of-truth rule
- `yai-specs` is **source of truth** for specs/contracts/formal/vectors.
- `yai` and `yai-cli` **consume pinned versions** (tag/commit) of `yai-specs`.

### Change Budget rule
- Phases 0.1.0-0.1.2 are **structure/mapping/links only**.
- Semantic changes (meaning of contracts, behavior expectations) are **not allowed** until the foundation gates exist (0.1.5+), unless explicitly broken out into a separate runbook.

### Closure rule
A phase is "closed" only when:
1) its gate passes locally and in CI,
2) the MP file is created and includes evidence,
3) the runbook links to that MP as closed.

---

## 6) Phases

> Each phase has: **Claim**, **Scope**, **Work**, **Gate (Exit Criteria)**, **Deliverables**, **MP**.

---

<a id="phase-0-1-0-canonical-tree"></a>
### 0.1.0 - Canonical Tree & Domain Separation
**Claim:** repository layout is canonical, navigable, and domain-separated.  
**Scope:** boundaries between `docs/`, `contracts/`, `specs/`, `formal/`, `compliance/`, `vectors/`, `tools/`.  
**Claim IDs:** `C-EVIDENCE-PACK-REPRODUCIBLE`  
**Mandatory evidence commands:**
- `tools/bin/yai-docs-trace-check --all`
**Work (typical):**
- ensure each domain has a landing README,
- ensure "source-of-truth pointers" exist (SPEC_MAP / REGISTRY),
- ensure no mixed-purpose folders, no duplicate "index-of-index".

**Gate (exit criteria):**
- tree is stable, no semantic rewrites,
- all "top-level" navigation entries exist and are consistent.

**Deliverables:**
- domain README refresh (no semantics),
- optionally: `docs/` navigation pointers.

**MP (planned):**
- `docs/20-governance/24-milestone-packs/specs-refactor-foundation/MP-SPECS-REFACTOR-FOUNDATION-0.1.0.md`

---

<a id="phase-0-1-1-mapping-only"></a>
### 0.1.1 - Pure Mapping (Move/Rename Only)
**Claim:** mapping cleanup is structural only (move/rename), no normative content changes.  
**Scope:** move/rename, deduplicate wrong placements, normalize paths.  
**Hard rule:** normative artifacts content must remain identical (byte-level if possible).
**Claim IDs:** `C-EVIDENCE-PACK-REPRODUCIBLE`  
**Mandatory evidence commands:**
- `tools/bin/yai-docs-trace-check --all`

**Work (typical):**
- move files to correct domains,
- remove duplicate copies and keep the canonical one,
- update internal pointers (but keep meaning unchanged).

**Gate:**
- no semantic delta in contracts/specs (review: mapping-only),
- link pointers updated accordingly.

**Deliverables:**
- moved/renamed files,
- updated internal references (path updates only).

**MP (planned):**
- `MP-SPECS-REFACTOR-FOUNDATION-0.1.1.md`

---

<a id="phase-0-1-2-sanity-links"></a>
### 0.1.2 - Sanity Link & Pointer Health
**Claim:** no broken links or ghost paths; indexes are coherent.  
**Scope:** markdown links, anchors, SPEC_MAP/REGISTRY references, pointer docs.  
**Claim IDs:** `C-EVIDENCE-PACK-REPRODUCIBLE`  
**Mandatory evidence commands:**
- `tools/bin/yai-docs-trace-check --all`
**Work (typical):**
- fix broken relative links,
- fix missing anchors,
- ensure SPEC_MAP and REGISTRY reflect real paths.

**Gate:**
- internal link check passes (at minimum locally; CI later becomes mandatory in 0.1.5),
- no dangling references.

**Deliverables:**
- corrected links/pointers,
- updated maps.

**MP (planned):**
- `MP-SPECS-REFACTOR-FOUNDATION-0.1.2.md`

---

<a id="phase-0-1-3-consumer-yai"></a>
### 0.1.3 - Consumer-Ready Wiring in `yai`
**Claim:** `yai` consumes `yai-specs` deterministically under the new structure.  
**Scope:** pins/tags/commit refs, include paths, build wiring, verify scripts.  
**Claim IDs:** `C-SPEC-FIRST-PINNED`, `C-EVIDENCE-PACK-REPRODUCIBLE`  
**Mandatory evidence commands:**
- `tools/release/check_pins.sh`
- `tools/bin/yai-proof-check`
**Work (typical):**
- update include paths to new canonical structure,
- ensure `tools/release/check_pins.sh` (or equivalent) gates drift,
- run `yai` build + verify.

**Gate:**
- `yai` build is green,
- `yai` verify suite is green,
- pin check is green.

**Deliverables:**
- consumer path changes in `yai`,
- pin updates.

**MP (planned):**
- `MP-SPECS-REFACTOR-FOUNDATION-0.1.3.md`

---

<a id="phase-0-1-4-consumer-yai-cli"></a>
### 0.1.4 - Consumer-Ready Wiring in `yai-cli`
**Claim:** `yai-cli` remains aligned and deterministic after specs mapping.  
**Scope:** pin, includes, references, vectors usage, CLI build/tests.  
**Claim IDs:** `C-SPEC-FIRST-PINNED`, `C-EVIDENCE-PACK-REPRODUCIBLE`  
**Mandatory evidence commands:**
- `tools/release/check_pins.sh`
- `tools/bin/yai-proof-check`
**Gate:**
- `yai-cli` build/tests green,
- pin check green,
- vectors/tests (if any) consistent.

**Deliverables:**
- consumer path changes in `yai-cli`,
- pin updates.

**MP (planned):**
- `MP-SPECS-REFACTOR-FOUNDATION-0.1.4.md`

---

<a id="phase-0-1-5-ci-guardrails"></a>
### 0.1.5 - CI Hard Guardrails (Enterprise)
**Claim:** PRs are **non-mergable** if any required check fails; logs are readable; checks are separated.  
**Scope:** `yai-specs` CI and repo gates.
**Claim IDs:** `C-EVIDENCE-PACK-REPRODUCIBLE`, `C-SKIP-FAIL-MANDATORY`  
**Mandatory evidence commands:**
- `tools/bin/yai-docs-trace-check --all`
- `tools/bin/yai-proof-check`

**Work (minimum deliverables):**
- Docs lint: markdownlint + internal link-check
- JSON validation: schema validation + vectors validation
- Verify registry: registry matches filesystem
- Doxygen build
- TLA smoke check (smoke only in this phase)
- `Makefile` target `ci` aggregates all
- `.github/workflows/ci.yml` with separate jobs

**Gate:**
- `make ci` passes locally (where applicable),
- CI workflow passes on PR,
- branch protection requires:
  - `CI / lint-docs`
  - `CI / validate-json`
  - `CI / verify-registry`
  - `CI / doxygen`
  - `CI / tla-smoke`

**Deliverables (canonical paths):**
- `.github/workflows/ci.yml`
- `.markdownlint.yml`, `.markdownlintignore`, `.lychee.toml`
- `tools/validate/lint_docs.sh`
- `tools/requirements-dev.txt`
- `vectors/schema/*` (schemas)
- `tools/validate/validate_json.py`
- registry validator path wired in CI
- `tools/formal/tla_smoke.sh`
- `Makefile` targets: `lint-docs`, `validate-json`, `verify-registry`, `doxygen`, `tla-smoke`, `ci`

**MP (planned):**
- `MP-SPECS-REFACTOR-FOUNDATION-0.1.5.md`

---

<a id="phase-0-1-6-toolchain-policy"></a>
### 0.1.6 - Internal Toolchain & Policy (Enterprise)
**Claim:** validate/format/policy/release operations are repeatable locally and in CI.  
**Scope:** toolchain scripts and policy gates.
**Claim IDs:** `C-EVIDENCE-PACK-REPRODUCIBLE`  
**Mandatory evidence commands:**
- `tools/bin/yai-docs-trace-check --all`
- `tools/bin/yai-proof-check`

**Work (minimum deliverables):**
- `tools/validate/validate_all.sh` aggregator
- Prettier-based formatting + `format-check`
- Stability policy: vectors cannot change without bump + changelog
- Release helpers: release notes generator + tag checklist
- CI wiring:
  - `format-check` job (fails if diff after formatting)
  - `policy` job

**Gate:**
- Local:
  - `make validate`
  - `make format`
  - `make format-check` (clean)
  - `make policy`
- CI:
  - `format-check` passes
  - `policy` passes
  - baseline CI jobs (from 0.1.5) still pass

**Deliverables:**
- `.prettierrc.json`, `.prettierignore`
- `tools/format/format_all.sh`
- `tools/validate/validate_all.sh`
- `tools/policy/check_stability.sh`
- `tools/release/generate_release_notes.sh`
- `tools/release/tag_checklist.md`
- Makefile targets: `validate`, `format`, `format-check`, `policy`, `release-notes`

**MP (planned):**
- `MP-SPECS-REFACTOR-FOUNDATION-0.1.6.md`

---

<a id="phase-0-1-7-formal-binding"></a>
### 0.1.7 - Formal Binding & Traceability Matrix (Enterprise)
**Claim:** contracts <-> specs <-> vectors <-> formal mapping is machine-verifiable.  
**Scope:** `formal/` bindings + traceability matrix + validators + CI gate.
**Claim IDs:** `C-AUTHORITY-SURFACE-RUNTIME`, `C-EVIDENCE-PACK-REPRODUCIBLE`  
**Mandatory evidence commands:**
- `tools/bin/yai-proof-check`
- `tools/bin/yai-docs-trace-check --all`

**Work (minimum deliverables):**
- Binding docs for each area:
  - `formal/bindings/README.md`
  - `formal/bindings/BINDING_PROTOCOL.md`
  - `formal/bindings/BINDING_VAULT.md`
  - `formal/bindings/BINDING_GRAPH.md`
  - `formal/bindings/BINDING_CONTROL.md`
  - `formal/bindings/BINDING_CLI.md`
  - `formal/bindings/BINDING_COMPLIANCE.md`
- Formal coverage landing:
  - `formal/spec_map.md` (coverage map, not a duplicate)
- Machine-readable matrix:
  - `formal/traceability.v1.json`
  - `formal/schema/traceability.v1.schema.json`
- Tooling validator:
  - `tools/formal/validate_traceability.py`
- Makefile + CI:
  - target `formal-coverage`
  - CI job `formal-coverage` required

**Gate:**
- `make formal-coverage` passes locally and in CI,
- every invariant has at least:
  - 1 binding + 1 spec artifact pointer (and vectors if they exist),
- no fake "formal=true" claims.

**MP (planned):**
- `MP-SPECS-REFACTOR-FOUNDATION-0.1.7.md`

---

<a id="phase-0-1-8-tla-reboot"></a>
### 0.1.8 - TLA Reboot & Model-Check CI (Enterprise)
**Claim:** TLA model checks are repeatable and staged:
- **quick** always runs on PR and stays green,
- **deep** runs on schedule/manual and is stabilized over time,
- traceability maps to real properties/configs.
**Claim IDs:** `C-AUTHORITY-SURFACE-RUNTIME`, `C-EVIDENCE-PACK-REPRODUCIBLE`  
**Mandatory evidence commands:**
- `tools/bin/yai-proof-check`
- `tools/bin/yai-docs-trace-check --all`

**Work (minimum deliverables):**
- Standard runner:
  - `tools/formal/run_tlc.sh` supporting `quick|deep`
- Clean configs:
  - `formal/configs/YAI_KERNEL.quick.cfg`
  - `formal/configs/YAI_KERNEL.deep.cfg`
- Reboot model (same top module name):
  - `formal/tla/YAI_KERNEL.tla` (helpers optional)
- Workflows:
  - `.github/workflows/formal.yml` (quick on PR/push; deep on schedule/dispatch)
- Docs:
  - `formal/README.md` (local run instructions)
- Traceability wiring:
  - `formal/traceability.v1.json` updated so invariants with formal coverage reference real properties in cfgs.

**Gate:**
- `make tla-quick` passes reliably,
- "Formal / quick" workflow is required check on PR,
- deep workflow runs on schedule/manual with logs uploaded as artifacts,
- traceability entries with `coverage.formal=true` match real properties/configs.

**MP (planned):**
- `MP-SPECS-REFACTOR-FOUNDATION-0.1.8.md`

---

## 7) Verification (commands)
> Commands differ slightly per repo. The point is: **each phase has a deterministic gate**.
> Mandatory check closure semantics: `SKIP = FAIL`.

### In `yai-specs` (foundation gates)
```bash
make ci
make validate
make format-check
make policy
make formal-coverage
make tla-quick
```

### In consumers (`yai`, `yai-cli`)

```bash
# pin governance (example)
./tools/release/check_pins.sh

# build + verify (examples)
make all
make verify
```

---

## 8) Failure Modes (common + fixes)

### Pin mismatch / consumer breaks after mapping

- **Symptom:** build failures due to includes/paths.
- **Fix:** update consumer wiring (0.1.3 / 0.1.4) + re-run pin checks + merge only when green.

### Docs link drift after moving files

- **Symptom:** broken anchors or broken relative links.
- **Fix:** finish 0.1.2 cleanly, then enforce with 0.1.5 lint-docs.

### Vectors drift without governance

- **Symptom:** vectors modified casually, consumers diverge.
- **Fix:** 0.1.6 policy gate enforces "vectors change => bump VERSION + CHANGELOG".

### Traceability lies ("formal=true" without properties)

- **Symptom:** traceability claims don't match TLC configs.
- **Fix:** keep formal=false until properties are real; optionally add a validator that checks cfg contains properties.

---

## 9) Rollback (safe rollback rules)

- Roll back to last known good `yai-specs` tag/commit in consumers.
- Revert mapping commits within the current phase only (don't mix phases in one revert blob).
- Re-run:
  - CI in `yai-specs`
  - pin checks and verify in consumers
  - then reopen the phase.

---

## 10) Governance: how this grows to 70-80 runbooks without dying

- Keep runbooks **small and phase-gated**, not "one giant forever document".
- Update a runbook only when:
  1. the phase definition was wrong, or
  2. gates/tools changed (new mandatory checks), or
  3. ADR/Proposal updated and changes the program constraints.
- Otherwise: create **new runbook** for new work, and reference old one.

> Runbook is like an ADR: it remains as record. You don't keep "growing it forever" unless the work is truly the same program.

---

## 11) References

- ADR: `docs/20-governance/22-adr/ADR-011-contract-baseline-lock.md`
- Specs: `deps/yai-specs/SPEC_MAP.md`, `deps/yai-specs/REGISTRY.md`
- Planned MPs: `docs/20-governance/24-milestone-packs/specs-refactor-foundation/*`
