---
id: RB-000
title: Runbook Template
status: template
owner: runtime
effective_date: 2026-02-18
revision: 1
supersedes: []
depends_on: []
related:
  adr: []
  specs: []
  test_plans: []
  tools: []
---

# RB-000 — Runbook Template (Operational)

This is an **operational runbook**. It defines an executable implementation plan:
- what changes must be made
- where (exact file targets)
- how to validate (tests + gates)
- what “done” means (acceptance checklist)

It is NOT an ADR:
- ADR = architectural decision + rationale + consequences
- Runbook = delivery plan + implementation sequence + verification

---

## 0) Identity

**Branch (required):** `feat/<area>-<topic>-vX`  
Examples:
- `feat/root-hardening-v2`
- `feat/workspaces-lifecycle-v1`
- `feat/engine-attach-v4`
- `feat/data-plane-v5`

**Scope level:** one runbook = one feature series; use sub-phases for atomic delivery (vX.0, vX.1...)

**Audience:** contributors implementing code (not just readers)

---

## 1) Objective

Write the objective as a single “non-negotiable outcome”.

Example:
- “Harden Root ↔ Kernel boundary with strict envelope-only enforcement, deterministic errors, and audit logs.”

---

## 2) Sequencing and Prerequisites

### 2.1 Execution Order (if part of a series)
Declare the position in the global sequence. Example:
1. Root hardening
2. Workspace lifecycle
3. Engine attach
4. Data plane
5. Mind Redis STM

### 2.2 Dependencies (hard prerequisites)
List what MUST already be true, or this runbook must not start.

- ✅ ws_id validation active in kernel
- ✅ path jail active for all runtime file ops
- ✅ multi-stream logger mandatory and running
- ✅ authority table closed for privileged commands (arming/role)

If a prerequisite cannot be verified, include the exact check:
- `yai verify core`
- `yai gate ws`
- `make verify`
- unit/integration test name

---

## 3) Definitions

### 3.1 Terms
Define only the terms that are critical for this runbook.
- ws_id
- arming
- role
- trace_id
- “runtime-bound command”

### 3.2 Invariants
List the invariants this runbook must not break.
Prefer linking to specs/contracts.

Example:
- I-001 Traceability (trace_id must survive end-to-end)
- I-003 Governance (privileged ops require arming+role)
- I-006 External Effect Boundary (effects only through governed planes)

---

## 4) Scope

### 4.1 In scope
Bullet list of what WILL be implemented.

### 4.2 Out of scope
Bullet list of what will NOT be touched (this prevents refactor creep).

---

## 5) Deliverables (Phased)

Break into sub-phases. Each phase must be:
- small enough to ship
- independently verifiable
- limited blast radius

For each phase use the exact structure below.

---

### vX.0 — <Phase Title>

**Branch:** `feat/<area>-<topic>-vX.0-<short>`  
**Goal:** one sentence.

#### 5.0.1 File Targets
List exact file paths to create/modify.

Example:
- `deps/yai-specs/storage/DATA_PLANE.md` (NEW)
- `kernel/include/storage_paths.h` (NEW)
- `kernel/src/core/storage_paths.c` (NEW)
- `docs/runbooks/data-plane.md` (UPDATE)
- `tools/python/yai_tools/verify/law_kernel.py` (UPDATE)

#### 5.0.2 Implementation Notes
Rules, patterns, and constraints. Keep code snippets minimal.
If you include code, it must be pattern-level, not full modules.

#### 5.0.3 Verification
Exact commands that must pass.

Example:
- `make verify`
- `make test`
- `tools/bin/yai-verify core`
- `tools/bin/yai-gate ws`
- `tools/bin/yai-suite l0-l2`

#### 5.0.4 Acceptance (vX.0)
- [ ] Spec committed and referenced
- [ ] Code compiles on supported platforms
- [ ] Verification commands pass
- [ ] Logs/audit show expected behavior
- [ ] No drift: files match declared layout/contract

---

### vX.1 — <Phase Title>

Repeat same structure.

---

## 6) Observability and Audit

Declare what must be logged, and where.

- Log streams: `kernel.log`, `engine.log`, `root.log` (if applicable)
- Mandatory metadata per log line:
  - ws_id (or “system”)
  - trace_id
  - component
  - decision outcome / error code

If audit artifacts are required (hashes, verify reports), list paths:
- `~/.yai/artifacts/...`
- `~/.yai/run/<ws_id>/logs/...`

---

## 7) Risk and Rollback

### 7.1 Risk Checklist
- protocol compatibility risk
- file layout migration risk
- performance risk
- data integrity risk

### 7.2 Rollback Strategy
Define the rollback approach:
- feature flag / env var
- compatibility stub
- keep old path + new path in parallel for one phase
- “hard break” allowed? (usually no)

---

## 8) Final Definition of Done (Runbook Complete)

This is the closure criteria for the entire series (not a single phase).

- [ ] All phases complete
- [ ] All gates/verifies/suites updated and passing
- [ ] ADR/spec links updated if architecture changed
- [ ] Docs updated: user-guide/dev-guide as needed
- [ ] No orphan scripts: tooling lives under `tools/`
- [ ] Compatibility shims removed (if planned)

---

## 9) References

Must be explicit and stable.

- ADR: `docs/design/adr/ADR-xxx-*.md`
- Specs: `deps/yai-specs/...`
- Test plans: `docs/test-plans/...`
- Tools: `tools/...` (preferred) or `scripts/compat/...` (temporary only)
