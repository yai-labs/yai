---
id: RB-WORKSPACES-LIFECYCLE
title: Workspaces Lifecycle
status: active
owner: runtime
effective_date: 2026-02-18
revision: 2
supersedes: []
depends_on:
  - RB-ROOT-HARDENING
adr_refs:
  - docs/20-program/22-adr/ADR-007-workspace-isolation.md
  - docs/20-program/22-adr/ADR-008-connection-lifecycle.md
decisions:
  - docs/20-program/22-adr/ADR-007-workspace-isolation.md
  - docs/20-program/22-adr/ADR-008-connection-lifecycle.md
related:
  adr:
    - docs/20-program/22-adr/ADR-007-workspace-isolation.md
    - docs/20-program/22-adr/ADR-008-connection-lifecycle.md
  specs:
    - deps/yai-law/specs/protocol/include/transport.h
    - deps/yai-law/specs/protocol/include/auth.h
    - deps/yai-law/specs/protocol/include/errors.h
    - deps/yai-law/specs/protocol/include/yai_protocol_ids.h
  test_plans:
    - docs/40-qualification/test-plans/hardfail.md
  tools:
    - tools/bin/yai-verify
    - tools/bin/yai-gate
    - tools/bin/yai-suite
tags:
  - runtime
  - workspace
---

# RB-WORKSPACES-LIFECYCLE — Workspace Commands With Real Side-Effects (YAI 0.1.x)

This runbook upgrades workspace commands from "stub OK" to **real, governed, deterministic side-effects** while preserving Root hardening.

Non-negotiable outcomes:
- `yai kernel ws create <id>` creates `~/.yai/run/<id>/` and minimal state files
- `yai kernel ws destroy <id>` removes deterministically, guarded against traversal
- Root remains a pure router (envelope validation + byte-perfect forward/relay)
- Kernel enforces policy and applies side-effects only when valid + authorized

## 1) Purpose

Upgrade workspace commands from stub behavior to deterministic governed side-effects while preserving Root hardening invariants.

## 2) Preconditions

- [x] Root hardening baseline is active (handshake gate, ws_id validation, deterministic errors).
- [x] Kernel is reachable from control plane commands.
- [x] `yai root ping` and one kernel command are already green.

## 3) Inputs

- Protocol anchors:
  - `deps/yai-law/specs/protocol/include/transport.h`
  - `deps/yai-law/specs/protocol/include/auth.h`
  - `deps/yai-law/specs/protocol/include/errors.h`
- Tooling:
  - `tools/bin/yai-verify`
  - `tools/bin/yai-gate`
  - `tools/bin/yai-suite`

## 4) Procedure

### Position in the global sequence

1. Root hardening ✅
2. Workspaces lifecycle ✅ (this document)
3. Engine attach
4. Data plane
5. Mind Redis STM

### Hard prerequisites

- Root hardening is complete enough that:
  - handshake gate is active
  - ws_id validation exists and is used
  - Root forwards byte-perfect to Kernel
  - deterministic error replies exist (no silent drop)
- Kernel boots and can respond to control plane requests
- CLI can run at least:
  - `yai root ping`
  - `yai kernel status` (or any kernel command that hits the kernel)

If these are not true, stop and complete RB-ROOT-HARDENING first.

---

### Scope

### In scope

- Implement Kernel handlers for:
  - `ws.create`
  - `ws.destroy`
  - `ws.list` (minimal)
- Ensure side effects happen ONLY under:
  - valid ws_id
  - successful handshake
  - correct authority for privileged ops (arming+role)
- Minimal on-disk workspace layout (0.1.x)
- Deterministic response behavior + logging for all outcomes

### Out of scope

- Data plane databases (LMDB/DuckDB) beyond a minimal manifest placeholder
- Engine attachment and engine per-workspace sockets
- Mind STM / Redis
- Expanding protocol schema or adding new envelope fields

---

### Operational Workflow (Daily)

### Clean runtime before each test round

```bash
pkill -f yai-root-server || true
pkill -f yai-kernel || true
pkill -f yai-boot || true
```

### Build + boot baseline

```bash
make clean
make
yai-boot --master
```

### Sanity checks

```bash
yai root ping
```

Expected:

- Root responds
- logs exist (root/kernel), and failures are deterministic

---

### Deliverables (Phased)

---

<a id="phase-0-1-0-workspace-layout"></a>
### 0.1.0 — Define Minimal Workspace Layout + Manifest Stub

**Branch:** `feat/workspaces-lifecycle-0.1.0-layout`  
**Goal:** decide and implement the minimal filesystem footprint created by `ws create`.

#### Minimal layout (0.1.x)

`~/.yai/run/<ws_id>/`

- `manifest.json` (required)
- `logs/` (directory; optional creation in 0.1.0, mandatory by later runbooks)

`manifest.json` (0.1.x stub, stable keys):

```json
{
  "ws_id": "testws",
  "created_at": "2026-02-18T00:00:00Z",
  "owner_role": "operator"
}
```

#### File targets

Specs/doc (optional but recommended even as a short note):

- `docs/20-program/23-runbooks/workspaces-lifecycle.md` (this file; update if needed)

Kernel paths/helpers:

- `kernel/src/core/project_tree.c` (if it already builds directories)
- or introduce:
  - `kernel/include/storage_paths.h` (NEW, if not already created by other runbooks)
  - `kernel/src/core/storage_paths.c` (NEW)

Ensure directory creation is jailed under `~/.yai/run/`.

#### Verification

Create a workspace:

```bash
yai kernel ws create testws
```

Confirm:

- `~/.yai/run/testws/manifest.json` exists and contains ws_id

#### Acceptance (0.1.0)

- [ ] minimal layout decision is written (in this runbook section is enough)
- [ ] `ws create` produces directory + manifest stub
- [ ] all paths are inside `~/.yai/run/<ws_id>/` only

---

<a id="phase-0-1-1-ws-create-guardrails"></a>
### 0.1.1 — Kernel Implements ws.create With Guardrails

**Branch:** `feat/workspaces-lifecycle-0.1.1-ws-create`  
**Goal:** `ws create` performs real side-effects only when input is valid and authorized.

#### File targets (read-first)

Specs:

- `deps/yai-law/specs/protocol/include/transport.h`
- `deps/yai-law/specs/protocol/include/yai_protocol_ids.h`
- `deps/yai-law/specs/protocol/include/errors.h`
- `deps/yai-law/specs/protocol/include/auth.h`

Kernel dispatch/codec/session:

- `kernel/src/core/rpc_binary.c` (or current dispatch entry)
- `kernel/src/core/rpc_codec.c` (if present)
- `kernel/src/core/yai_session.c`
- `kernel/src/core/transport.c` or `kernel/src/core/project_tree.c` (filesystem ops)

CLI command assembly (so you know what payload looks like):

- wherever kernel ws commands are built (CLI source in this repo)

#### Rules (hard)

No side effects if:

- ws_id is invalid
- ws_id empty
- handshake not established (if applicable to your current session model)
- authority is insufficient for this command (see 0.1.3)

Deterministic response always:

- on success: OK response frame
- on failure: error response frame, with stable numeric code

#### C correctness note (must fix if present)

Do not check ws_id as pointer:

```c
if (!env->ws_id || strlen(env->ws_id) == 0)   // WRONG
```

Use:

```c
if (env->ws_id[0] == '\0')                    // RIGHT
```

#### Verification

```bash
yai kernel ws create testws
ls -la ~/.yai/run/testws
cat ~/.yai/run/testws/manifest.json
```

#### Acceptance (0.1.1)

- [ ] `ws create` creates `~/.yai/run/<ws_id>/`
- [ ] manifest stub is created deterministically
- [ ] invalid ws_id produces deterministic reject and creates nothing

---

### 0.1.2 — Kernel Implements ws.list (Minimal)

**Branch:** `feat/workspaces-lifecycle-0.1.2-ws-list`  
**Goal:** list existing workspaces deterministically (even a simple dir scan is fine in 0.1.x).

#### File targets

Kernel:

- `kernel/src/core/transport.c` / `project_tree.c` (dir scan utilities)
- `kernel/src/core/rpc_binary.c` (dispatch)

CLI:

- ws list command if missing

#### Rules

- Only list directories under `~/.yai/run/`
- Must ignore non-directories and invalid names (apply ws_id validator)

#### Verification

```bash
yai kernel ws create testws
yai kernel ws list
```

#### Acceptance (0.1.2)

- [ ] `ws list` returns at least `testws`
- [ ] output is deterministic and filtered by validator

---

### 0.1.3 — ws.destroy + Authority Enforcement (arming+role)

**Branch:** `feat/workspaces-lifecycle-0.1.3-ws-destroy-authority`  
**Goal:** destroy is privileged and must be envelope-governed; Kernel must re-check even if Root does.

#### File targets

Specs:

- `deps/yai-law/specs/protocol/include/auth.h`
- `deps/yai-law/specs/protocol/include/roles.h` (if present)

Root (only if you mirror policy fast-fail; Kernel is mandatory):

- `root/src/yai_root_server.c`

Kernel:

- `kernel/src/core/rpc_binary.c`
- `kernel/src/core/yai_session.c`
- filesystem removal code location (`transport.c` / `project_tree.c`)

#### Policy (0.1.x baseline)

`ws.destroy` requires:

- `arming=1`
- `role>=operator`

`ws.create` can be either:

- privileged (same as destroy) OR
- allowed for `role>=user` with arming optional

Choose ONE policy and document it here (do not drift).

Recommended for safety during early 0.1.x: make both create and destroy privileged until you formalize tenancy.

#### Guardrails (critical)

Deletion must be jailed:

- target path must be exactly `~/.yai/run/<ws_id>/`
- refuse if ws_id invalid (no attempt)
- refuse if computed path escapes jail

Deletion must be deterministic:

- if workspace missing: return NOTFOUND code (or OK with "already absent"), but pick one and keep it stable
- always log outcome

#### Verification

```bash
yai kernel ws create testws

# should fail without authority
yai kernel ws destroy testws

# should pass with authority (depending on your CLI flags model)
yai kernel ws destroy testws --arming --role operator

test ! -d ~/.yai/run/testws && echo OK
```

#### Acceptance (0.1.3)

- [ ] destroy without arming rejects deterministically
- [ ] destroy with arming+operator succeeds
- [ ] deletion cannot traverse paths (no rm -rf outside jail)

---

### 0.1.4 — Torture + Repeatability (Workspace + Protocol)

**Branch:** `feat/workspaces-lifecycle-0.1.4-torture`  
**Goal:** prove workspace lifecycle correctness with repeatable tests.

#### Minimum test cases

1. handshake ok
2. handshake wrong version → reject
3. ws create valid → creates dir + manifest
4. ws create invalid id → reject + no side effects
5. ws destroy without arming → reject
6. ws destroy with arming+role → ok
7. payload_len > max → reject
8. magic/version wrong → reject

#### Tools

Preferred (move toward tools/):

- `tools/bin/yai-gate ws`
- `tools/bin/yai-suite` (add a workspace sub-suite)

Temporary compatibility allowed:

- `tools/...` only as shim; do not add new logic there

#### Verification

One command that runs all cases and prints PASS/FAIL per case.

#### Acceptance (0.1.4)

- [ ] tests are repeatable on a clean runtime
- [ ] every failure produces deterministic error codes and logs

---

## 5) Verification

Minimum log expectations:

- Root logs FORWARD/REJECT decisions (already covered by RB-ROOT-HARDENING)
- Kernel logs:
  - ws.create attempt + outcome
  - ws.destroy attempt + outcome
  - filesystem path actually used (after jail resolution)

Always include ws_id + trace_id when available.

---

## 6) Failure Modes

- Symptom: workspace side-effects appear outside `~/.yai/run/<ws_id>`.
  - Fix: enforce path jail and ws_id validation before applying FS mutations.
- Symptom: create/destroy behavior is non-deterministic across retries.
  - Fix: harden idempotency paths and rerun lifecycle checks.
- Symptom: unauthorized workspace operations pass.
  - Fix: reapply envelope authority gate (`arming` + `role`) in kernel handlers.

## 7) Rollback

Rollback is phase-based:

- each phase is merged only after its acceptance passes
- if a phase regresses:
  - revert the phase branch merge
  - do not "patch-forward" in later phases

---

## 8) References

### Upstream proposals

- `docs/20-program/21-rfc/RFC-003-workspace-lifecycle-and-isolation.md`

### Milestone packs

- `docs/20-program/24-milestone-packs/workspaces-lifecycle/MP-WORKSPACES-LIFECYCLE-0.1.0.md` *(planned)*
- `docs/20-program/24-milestone-packs/workspaces-lifecycle/MP-WORKSPACES-LIFECYCLE-0.1.1.md` *(planned)*

## 9) Final Definition of Done

- [x] `yai kernel ws create testws` creates `~/.yai/run/testws/manifest.json`
- [x] `yai kernel ws list` lists created workspaces deterministically
- [x] `yai kernel ws destroy testws` requires authority and deletes only inside jail
- [x] invalid ws_id is rejected deterministically with zero side effects
- [x] workspace+protocol torture suite passes repeatably

## Traceability

- ADR refs:
  - `docs/20-program/22-adr/ADR-007-workspace-isolation.md`
  - `docs/20-program/22-adr/ADR-008-connection-lifecycle.md`
- Law/spec refs:
  - `deps/yai-law/specs/protocol/include/transport.h`
  - `deps/yai-law/specs/protocol/include/auth.h`
  - `deps/yai-law/specs/protocol/include/errors.h`
- MPs:
  - `docs/20-program/24-milestone-packs/workspaces-lifecycle/MP-WORKSPACES-LIFECYCLE-0.1.0.md` *(planned)*
  - `docs/20-program/24-milestone-packs/workspaces-lifecycle/MP-WORKSPACES-LIFECYCLE-0.1.1.md` *(planned)*

## 10) Operational Closure

This runbook is operationally closed for the current phase and remains active as a prerequisite baseline for `RB-ENGINE-ATTACH`.
