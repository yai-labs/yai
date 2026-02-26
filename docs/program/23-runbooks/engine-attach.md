---
id: RB-ENGINE-ATTACH
title: Engine Attach
status: active
owner: runtime
effective_date: 2026-02-19
revision: 2
supersedes: []
depends_on:
  - RB-ROOT-HARDENING
  - RB-WORKSPACES-LIFECYCLE
adr_refs:
  - docs/20-program/22-adr/ADR-009-engine-attachment.md
  - docs/20-program/22-adr/ADR-008-connection-lifecycle.md
decisions:
  - docs/20-program/22-adr/ADR-009-engine-attachment.md
  - docs/20-program/22-adr/ADR-008-connection-lifecycle.md
related:
  adr:
    - docs/20-program/22-adr/ADR-009-engine-attachment.md
    - docs/20-program/22-adr/ADR-008-connection-lifecycle.md
  specs:
    - deps/yai-law/specs/protocol/include/transport.h
    - deps/yai-law/specs/protocol/include/yai_protocol_ids.h
  test_plans:
    - docs/40-qualification/test-plans/hardfail.md
  tools:
    - tools/bin/yai-verify
    - tools/bin/yai-gate
tags:
  - runtime
  - engine
---

# RB-ENGINE-ATTACH — Engine Attach

## 1) Purpose
Bring L2 Engine inside the governed Root/Kernel control path using ADR-009 shared runtime-plane semantics and deterministic protocol behavior.

## 2) Preconditions
- [x] Workspace lifecycle baseline is active.
- [x] Root boundary hardening is already green.
- [x] Protocol IDs and error mapping can be updated in sync with specs.

## 3) Inputs
- Runtime targets: `root`, `kernel`, `engine`, `yai-cli`
- Spec anchors: protocol IDs, transport, error codes
- Validation tooling: `tools/bin/yai-verify`, `tools/bin/yai-gate`

## 4) Procedure
Execute the phased attach sequence in this document (ADR decision first, then kernel lifecycle commands, then CLI/operator flows).

## 5) Verification
- Run pre-flight and per-step acceptance checks.
- Confirm deterministic start/stop/status semantics and functional attach via Root/Kernel-mediated RPC probes.

## 6) Failure Modes
- Symptom: engine RPC probe fails even if control socket is present/missing.
  - Fix: treat socket exposure as informational and gate readiness on RPC probe success.
- Symptom: mismatched authority behavior between kernel and CLI.
  - Fix: enforce envelope authority checks (`arming` + `role`) on server side and align client expectations.

## 7) Rollback
- Stop engine processes, clean workspace runtime artifacts, and revert only the active phase changes.
- Re-run baseline ping and kernel status checks before retry.

## 8) References
- ADR: `docs/20-program/22-adr/ADR-009-engine-attachment.md`
- Runbooks: `docs/20-program/23-runbooks/root-hardening.md`, `docs/20-program/23-runbooks/workspaces-lifecycle.md`
- Test plans: `docs/40-qualification/test-plans/hardfail.md`

## Traceability
- ADR refs:
  - `docs/20-program/22-adr/ADR-009-engine-attachment.md`
  - `docs/20-program/22-adr/ADR-008-connection-lifecycle.md`
- MPs (planned):
  - `docs/20-program/24-milestone-packs/engine-attach/MP-ENGINE-ATTACH-0.1.0.md` (ID: `MP-ENGINE-ATTACH-0.1.0`)

## Appendix — Detailed Operational Notes (Legacy Detailed Content)

> NOTE: This appendix contains transitional notes. Canonical topology is ADR-009 shared engine plane; do not infer per-workspace engine process requirements from older snippets below.


### YAI L2 Engine Attach v4 — Operational Runbook

**Branch:** `feat/l2-engine-attach-v1`  
**Objective:** Bring L2 Engine into L0↔L1 governed pipeline with shared runtime-plane topology and workspace-scoped dispatch metadata.

---

<a id="phase-engine-attach-v4"></a>
## Objective (v4)

Bring L2 Engine "inside" the governed L0↔L1 pipeline:
- Root governs ingress/routing
- Kernel enforces authority and workspace isolation
- Engine executes as a **shared runtime plane** with workspace context carried in dispatch metadata
- Everything remains consistent with wire protocol (envelope invariants + error replies)

**Final outcome (end v4):**
- Runtime topology remains `boot -> root -> kernel -> engine`
- `yai engine --ws <id> ...` requests are authorized by Kernel and executed in workspace context
- Engine attach/readiness is verified by functional RPC probe (not by per-workspace process/socket topology)
- Root remains pure router (does not interpret payload)

---

## Pre-flight (Always)

```bash
make clean
make
pkill -f yai-root-server || true
pkill -f yai-boot || true
pkill -f yai-engine || true
yai-boot --master
yai root ping
```

---

## STEP 0: ADR (Socket Layout L2 Decision) **MANDATORY**

### Target file
- `docs/20-program/22-adr/ADR-009-engine-attachment.md`

### Canonical schema (ADR-009)

Engine attachment is a **shared runtime plane** under Root governance.
Workspace context is passed through dispatch metadata, not process topology.

- Engine control socket exposure is optional and informational.
- Readiness must be validated functionally via Root/Kernel-mediated RPC probe with `--ws <id>`.
- Do not require per-workspace engine process/socket as an acceptance condition.

### Acceptance
- [ ] ADR committed before code

---

## STEP 1: Law/Spec Add Engine Paths + IDs (If Missing)

### Files to read FIRST
- `deps/yai-law/specs/protocol/yai_protocol_ids.h`
- `deps/yai-law/specs/protocol/transport.h`
- `deps/yai-law/specs/protocol/errors.h` (optional)

### Deliverables

**1. Protocol IDs for Engine control (if don't exist)**
- `YAI_CMD_ENGINE_START`
- `YAI_CMD_ENGINE_STOP`
- `YAI_CMD_ENGINE_STATUS`

**2. Error codes used by Kernel when start/stop fails**
- `YAI_E_ENGINE_ALREADY_RUNNING`
- `YAI_E_ENGINE_NOT_RUNNING`
- `YAI_E_ENGINE_SPAWN_FAILED`
- `YAI_E_ENGINE_SOCKET_MISSING`

### Acceptance
- [ ] Build passes and CLI can send dedicated command_id

---

## STEP 2: Kernel Implement Engine Lifecycle Commands (L1 Control-Plane)

### Files to read FIRST
- `kernel/src/core/yai_session.c` (or where workspace+paths logic lives)
- `kernel/src/core/rpc_binary.c` (command dispatch)
- `kernel/src/core/transport.c` (path/spawn helpers if there)
- `kernel/src/enforcement/enforcement.c` (if JSON parsing here)

### 2.1 Path Invariants (ALWAYS use ws->run_dir)

- Engine runtime socket (if exposed): `~/.yai/run/engine/control.sock` (informational)
- Engine logs can remain global/runtime-plane while preserving `ws_id` in every request/trace.
- Workspace isolation is enforced by Kernel authority + dispatch metadata, not per-workspace engine process layout.

### 2.2 start Command

**Command:** shared-plane attach via `yai up --ws <control_ws> --detach --allow-degraded`

**Rules:**
- control workspace valid (default `dev`)
- **Authority:**
  - Requires `arming=1` and `role>=operator` for governed calls
- Root + Kernel must be reachable (`yai root ping`, `yai kernel --arming --role operator ping`)
- Engine readiness validated functionally via RPC probe in target workspace context

### 2.3 status Command

**Command:** `yai status` / `yai doctor` plus engine RPC probe semantics

**Rules:**
- `engine.socket_exposed` is informational only
- READY requires `root_ping=true`, `kernel_ping=true`, and `engine.rpc_ok=true`
- `engine.rpc_ok` is validated through governed `yai engine --ws <id> ...` call path

### 2.4 stop Command

**Command:** `yai down --ws <control_ws> --force`

**Rules:**
- stop is control-plane scoped; no per-workspace engine process lifecycle requirement
- post-stop status must return `overall=DEGRADED`

### Payload Responses

Always valid frame + short JSON:
- **status:** includes `engine.rpc_ok` and `engine.socket_exposed`
- **doctor:** includes same status snapshot and actionable hints

### Acceptance (kernel-side)
- [ ] Commands respond deterministically even on error

---

## STEP 3: Engine Shared Plane + Workspace-Scoped Dispatch

### Files to read FIRST
- `engine/src/main.c`
- `engine/src/core/transport.c` (or equivalent server listen)
- `engine/src/bridge/transport_client.c` (only if handshake to kernel needed)

### Deliverables

**1. Engine is attached as shared runtime plane:**
- single engine runtime plane process under Root/KERNEL governance
- workspace context comes from dispatch metadata per request

**2. Engine socket exposure (optional):**
- if exposed, path is `~/.yai/run/engine/control.sock`
- do not treat socket exposure as required readiness for qualification

**3. Handshake gate (minimum):**
- First request must be `YAI_CMD_HANDSHAKE`
- If not handshake → deterministic reject (`YAI_E_NEED_HANDSHAKE`)
- Note: if engine has no real commands yet, just ping/handshake is enough

**4. Logging:**
- Runtime logs must preserve `ws_id` per request/trace.
- Shared engine logs are acceptable as long as workspace context remains explicit.

### Acceptance
- [ ] Engine attach validated via RPC probe (`yai engine --ws <id> ...`) through Root/Kernel path

---

## STEP 4: Root Routing to Kernel Unchanged (No "Smart" Changes)

### Target file
- `kernel/src/bin/yai_root_server.c`

### Objective
- Root continues byte-perfect forward to Kernel
- Root does NOT talk to Engine (for now). Engine is managed by Kernel.

### Acceptance
- [ ] Root log shows forward of `ENGINE_*` to kernel

---

## STEP 5: CLI Commands (Shared-Plane + Workspace-Scoped Calls)

### Files to read FIRST
- `tools/cli/src/cmd_engine.c`
- `tools/cli/src/rpc.c`
- `tools/cli/src/paths.c` (only if you need ws run dir, but better not)

### Deliverables
- `yai up --ws dev --detach --allow-degraded`
- `yai status --json`
- `yai doctor --json`
- `yai engine --ws testws --arming --role operator storage get_node '{"id":"__yai_status_probe__"}'`
- `yai down --ws dev --force`

### Authority
**start/stop must set:**
- `arming=1`
- `role=operator` (or higher)

### Practical note
- The ws_id used to talk to root plane remains `system` (as you have now)
- The target workspace (`testws`) goes in the **payload** of the command, or as dedicated field if you already do this
- Root/Kernel will use envelope ws_id for "control" session (system), but operation concerns `target_ws` in payload
- (For absolute clarity: in v5 we'll do separate "control plane protocol", but not now)

### Acceptance
```bash
yai up --ws dev --detach --allow-degraded
yai status --json
yai doctor --json
yai engine --ws testws --arming --role operator storage get_node '{"id":"__yai_status_probe__"}'
yai down --ws dev --force
```

---

## STEP 6: Test Matrix v4 (End-to-End)

### Recommended script
- `tests/integration/test_l2_engine.sh`

### Minimum cases
1. runtime down → `status.overall=DEGRADED`
2. runtime up + root/kernel ping OK
3. engine socket not exposed but `engine.rpc_ok=true` → READY
4. engine RPC probe fails in target ws → deterministic fail
5. invalid ws_id in payload → deterministic reject

### Manual equivalent commands
```bash
yai up --ws dev --detach --allow-degraded
yai status --json
yai doctor --json
yai engine --ws testws --arming --role operator storage put_node '{"id":"attach-probe","kind":"probe","meta":{}}'
yai engine --ws testws --arming --role operator storage get_node '{"id":"attach-probe"}'
yai down --ws dev --force
```

---

## STEP 7: Hardening Micro-Fixes (Worth Gold)

**1. Remove `env->ws_id` not pointer warning:**
- Replace everywhere `!env->ws_id` with `env->ws_id[0]=='\0'`

**2. No silent truncating `strncpy` for socket path:**
- If `snprintf` >= cap → fail hard with error code (no connect/bind)

**3. Log directories create-on-demand:**
- If missing `~/.yai/run/<ws>/` → fail (don't create secretly, ws create does it)

### Acceptance
- [ ] Clean build and "deterministic" errors, never undefined behavior

---

## Definition of Done (v4)

- [ ] Engine attach validated via governed RPC probe in target workspace context
- [ ] `status/doctor` READY semantics include `engine.rpc_ok`
- [ ] Engine socket exposure treated as informational, not gating
- [ ] Authority and workspace isolation enforced by Root/Kernel boundaries
- [ ] Test script PASS
- [ ] ADR present and respected

---

## Next Steps

When starting v4 tomorrow, first thing to share: what does the payload look like that `cmd_engine.c` sends today (even if stub)? This way I can tell you exactly how to model `ENGINE_START/STOP/STATUS` kernel-side without "breaking" the contract you already have.

## 9) Operational Closure

Engine attach is closed for the current phase and is now the active operational baseline after `RB-WORKSPACES-LIFECYCLE`.

Closure sequence completed:
- workspace lifecycle gate confirmed
- engine attach lifecycle adopted as active runbook
- follow-up hardening remains tracked under `RB-ROOT-HARDENING` and `RB-DATA-PLANE`
