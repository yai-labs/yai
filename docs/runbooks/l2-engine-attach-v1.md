# YAI L2 Engine Attach v4 — Operational Runbook

**Branch:** `feat/l2-engine-attach-v1`  
**Objective:** Bring L2 Engine into L0↔L1 governed pipeline with workspace-bound lifecycle management.

---

## Objective (v4)

Bring L2 Engine "inside" the governed L0↔L1 pipeline:
- Kernel becomes **control authority** for start/stop/status Engine per workspace
- Engine becomes **workspace-bound** (ws_id mandatory) and **handshake-gated**
- Everything remains consistent with wire protocol (envelope invariants + error replies)

**Final outcome (end v4):**
- `yai kernel ws create testws` creates run dir
- `yai engine start testws` spawns `yai-engine` for that ws and creates `engine.pid`
- `yai engine status testws` confirms socket+pid
- `yai engine stop testws` stops engine and cleans up
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
- `docs/adr/000X-engine-socket-layout.md`

### Choose ONE schema and commit to it

**Option A (recommended for now): Per-workspace engine socket**
- Engine listens: `~/.yai/run/<ws_id>/engine/control.sock`
- **Pros:**
  - Strong tenant isolation
  - Mental clarity: everything for ws lives under its run dir
- **Cons:**
  - More path handling

**Option B: Global engine socket + ws_id in frame**
- Engine listens: `~/.yai/run/engine/control.sock`
- **Pros:**
  - Single socket, simpler
- **Cons:**
  - Weaker isolation (but ws_id in envelope still governs)

**Recommended decision v4:** Option A (per-workspace)  
It's "ICE-grade" and avoids a thousand ambiguities later.

### Acceptance
- [ ] ADR committed before code

---

## STEP 1: Law/Spec Add Engine Paths + IDs (If Missing)

### Files to read FIRST
- `law/specs/protocol/yai_protocol_ids.h`
- `law/specs/protocol/transport.h`
- `law/specs/protocol/errors.h` (optional)

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

**If Option A:**
- Engine socket: `ws->run_dir + "/engine/control.sock"`
- Engine pid: `ws->run_dir + "/engine.pid"`
- Engine log: `ws->run_dir + "/engine.log"` (optional but recommended)

**Create engine dir on start:**
```bash
mkdir -p ~/.yai/run/<ws>/engine
```

### 2.2 start Command

**Command:** `YAI_CMD_ENGINE_START`

**Rules:**
- ws_id valid
- Workspace run dir exists (or created in v3)
- **Authority:**
  - Requires `arming=1` and `role>=operator` (envelope-only)
- If `engine.pid` exists and process alive → `YAI_E_ENGINE_ALREADY_RUNNING`
- Spawn `yai-engine --ws <ws_id>` (or env `YAI_WS_ID=<ws_id>`)
- Write pid to `engine.pid`
- Brief wait (poll) for socket to appear (`control.sock`), otherwise fail with cleanup

### 2.3 status Command

**Command:** `YAI_CMD_ENGINE_STATUS`

**Rules:**
- ws_id valid
- If pid file missing → NOT RUNNING
- If pid not alive → NOT RUNNING (and clean stale pid)
- If socket missing → "degraded" state or deterministic error

### 2.4 stop Command

**Command:** `YAI_CMD_ENGINE_STOP`

**Rules:**
- Authority `arming=1 role>=operator`
- **Graceful kill:**
  - `SIGTERM`, brief wait, then `SIGKILL` fallback (only if needed)
- **Cleanup:**
  - Remove `engine.pid`
  - Optional: remove `engine/control.sock` (unlink)

### Payload Responses

Always valid frame + short JSON:
- **start ok:** `{"ok":true,"ws":"testws","pid":1234}`
- **status:** `{"ok":true,"ws":"testws","running":true,"pid":1234,"socket":true}`
- **stop ok:** `{"ok":true,"ws":"testws","stopped":true}`

### Acceptance (kernel-side)
- [ ] Commands respond deterministically even on error

---

## STEP 3: Engine Become Workspace-Bound + Socket Listen

### Files to read FIRST
- `engine/src/main.c`
- `engine/src/core/transport.c` (or equivalent server listen)
- `engine/src/bridge/transport_client.c` (only if handshake to kernel needed)

### Deliverables

**1. Engine requires ws_id:**
- Arg `--ws <id>` (preferred)
- Fallback env `YAI_WS_ID`
- If missing → non-zero exit + log to stderr

**2. Engine creates socket path (Option A):**
- `~/.yai/run/<ws_id>/engine/control.sock`
- `unlink` before bind
- `listen` and log "Runtime Plane online (path) ws=<id>"

**3. Handshake gate (minimum):**
- First request must be `YAI_CMD_HANDSHAKE`
- If not handshake → deterministic reject (`YAI_E_NEED_HANDSHAKE`)
- Note: if engine has no real commands yet, just ping/handshake is enough

**4. Logging:**
- If started by kernel, write to `~/.yai/run/<ws>/engine.log`
- If can't, at least stderr (kernel can redirect)

### Acceptance
- [ ] `yai-engine --ws testws` actually creates the socket

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

## STEP 5: CLI Commands `yai engine start|stop|status <ws>`

### Files to read FIRST
- `tools/cli/src/cmd_engine.c`
- `tools/cli/src/rpc.c`
- `tools/cli/src/paths.c` (only if you need ws run dir, but better not)

### Deliverables
- `yai engine start testws`
- `yai engine status testws`
- `yai engine stop testws`

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
yai kernel ws create testws
yai engine start testws
yai engine status testws
yai engine stop testws
yai kernel ws destroy testws
```

---

## STEP 6: Test Matrix v4 (End-to-End)

### Recommended script
- `scripts/test_l2_engine.sh`

### Minimum cases
1. start on non-existent ws → deterministic fail
2. start on valid ws → ok, socket present
3. double start → `ENGINE_ALREADY_RUNNING`
4. status running → running true
5. stop without arming → reject
6. stop with arming/operator → ok
7. status after stop → running false
8. invalid ws_id (in target payload) → deterministic reject

### Manual equivalent commands
```bash
yai kernel ws create testws
yai engine start testws
ls -la ~/.yai/run/testws/engine/control.sock
yai engine status testws
yai engine stop testws
yai engine status testws
yai kernel ws destroy testws
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

- [ ] Engine startable per workspace via Kernel
- [ ] Engine socket present under `~/.yai/run/<ws>/engine/`
- [ ] CLI has start/stop/status
- [ ] Envelope-only authority active on start/stop
- [ ] Test script PASS
- [ ] ADR present and respected

---

## Next Steps

When starting v4 tomorrow, first thing to share: what does the payload look like that `cmd_engine.c` sends today (even if stub)? This way I can tell you exactly how to model `ENGINE_START/STOP/STATUS` kernel-side without "breaking" the contract you already have.