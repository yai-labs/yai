# YAI Workspaces Lifecycle v3 — Operational Runbook

**Branch:** `feat/workspaces-lifecycle-v1`  
**Objective:** Transition workspace commands from "stub OK" to "real side-effects" with L0↔L1 hardening maintained.

---

## Objective

Bring workspace commands from "stub OK" to "real side-effects":
- `yai kernel ws create <id>` actually creates `~/.yai/run/<id>/` (with minimal files)
- `yai kernel ws destroy <id>` removes deterministically (with guardrails)
- Root remains pure router: validates envelope + byte-perfect forward to Kernel
- Kernel does enforcement (defense-in-depth) and applies side-effects only if valid/authorized

---

## Pre-flight (Always the Same)

```bash
make clean
make
pkill -f yai-root-server || true
pkill -f yai-boot || true
yai-boot --master
yai root ping
```

---

## STEP 0: Protocol Robustness Minimum (If Not Already Done in v2)

### Files to read FIRST
- `deps/yai-specs/protocol/transport.h`
- `deps/yai-specs/protocol/yai_protocol_ids.h`
- `deps/yai-specs/protocol/errors.h` (create if doesn't exist)

### Deliverables

**1. Standard error codes (numeric) used everywhere (root+kernel+cli)**
- `YAI_E_BAD_MAGIC`
- `YAI_E_BAD_VERSION`
- `YAI_E_BAD_WS_ID`
- `YAI_E_NEED_HANDSHAKE`
- `YAI_E_PAYLOAD_TOO_BIG`
- `YAI_E_ARMING_REQUIRED`
- `YAI_E_ROLE_REQUIRED`
- `YAI_E_BAD_CHECKSUM` (even if checksum not active today)

**2. Frame invariant rules**
- `payload_len <= YAI_MAX_PAYLOAD`
- `arming ∈ {0,1}`
- `role` in known range
- `ws_id` always validated with shared function (see STEP 2)

**3. Error reply always as valid frame**
- Valid envelope
- Short JSON payload like: `{"ok":false,"code":123,"msg":"bad_ws_id"}`

---

## STEP 1: Root Forward CONTROL to Kernel (No More "ok" Stub)

### Why
Today `ws create/destroy` responds OK but creates nothing because Root is not forwarding to Kernel.

### Files to read FIRST
- `kernel/src/bin/yai_root_server.c`
- `kernel/src/core/control_transport.c` (for frame read/write helpers)
- `kernel/src/core/rpc_binary.c` (if relay logic already there)
- `deps/yai-specs/protocol/transport.h`

### Implementation

**Root accepts only:**
- `HANDSHAKE` (always)
- Other commands ONLY after handshake

**Root validates invariants:**
- magic/version/len/ws_id/arming-role range

**Root does byte-perfect relay to Kernel (kernel plane socket):**
- Writes identical envelope + identical payload to Kernel
- Reads envelope+payload response from Kernel
- Rewrites identical to client

**Root does not:**
- Interpret payload
- Mutate trace_id
- Mutate ws_id

### Acceptance
- `yai kernel ws create testws` produces log in root: "FORWARD cmd=... ws=system"
- See Kernel receive/respond (kernel log or temporary debug)

---

## STEP 2: Centralize ws_id Validation (SINGLE Definition)

### Target file
- `deps/yai-specs/protocol/transport.h`

### Add
`static inline int yai_ws_id_is_valid(const char *s)` with strong rule:
- Length: 1..35
- Charset: `[A-Za-z0-9_-]`
- Forbid: `/` `~` spaces
- (Optional) forbid initial `.`

### Used by
- Root
- Kernel
- CLI
- (Later) Engine

### Acceptance
Root and Kernel deterministically reject:
- `"ws_id":"../../x"`
- `"ws_id":"~blah"`
- `"ws_id":"a b"`

---

## STEP 3: Kernel Actually Implement ws.create/ws.destroy/ws.list

### Files to read FIRST
- `kernel/src/core/yai_session.c`
- `kernel/src/core/rpc_binary.c` (or wherever dispatch arrives)
- `kernel/src/enforcement/enforcement.c` (if already parsing JSON payload)
- `kernel/src/core/transport.c` (paths / run dir)

### Practical method (zero client rework)

**1. Read payload that CLI sends for `yai kernel ws create testws`**
- File to open: `tools/cli/src/cmd_kernel.c` (or where JSON is composed)

**2. Kernel must recognize operation (even with minimal parsing) and do:**

**For `create`:**
- Create `~/.yai/run/<ws_id>/`
- Create `semantic.sqlite` (even empty or "touch" for now)
- Create `lock` / `kernel.pid` if already planned

**For `destroy`:**
- Remove safely ONLY inside `~/.yai/run/<ws_id>/` (never path traversal)

**For `list`:**
- List present workspaces (even rough: dir scan)

### Fundamental guardrail
- No side-effects if ws_id invalid
- Response always deterministic (ok/error code)

### Real warning fix (you saw this)
Replace:
```c
if (!env->ws_id || strlen(env->ws_id) == 0)
```
With:
```c
if (env->ws_id[0] == '\0')
```

### Acceptance
```bash
yai kernel ws create testws
ls -la ~/.yai/run/testws
yai kernel ws destroy testws
test ! -d ~/.yai/run/testws && echo OK
```

---

## STEP 4: Authority Enforcement Envelope-Only (Defense in Depth)

### Files to read FIRST
- `deps/yai-specs/protocol/auth.h` (or define policy table there)
- Root + Kernel (same command matrix → required role/arming)

### Rule
- Root blocks already, Kernel re-blocks anyway
- Enforcement ONLY on envelope metadata (command_id, role, arming), never on payload

### Practical note
If you start requiring `arming=1 role=operator` for `ws destroy/create`, then:
- Update CLI (cmd_kernel) to set authority before call
- Or add flags `--arming --role operator` CLI-side

### Acceptance
- `yai kernel ws destroy testws` without arming → FAIL (code "arming required")
- With arming+operator → OK

---

## STEP 5: Test Matrix "Workspace + Protocol"

### Mandatory minimum
- Handshake ok
- Handshake wrong version
- ws create valid → creates dir
- ws create invalid id → deterministic reject
- ws destroy without arming → reject
- payload_len > max → reject
- magic/version wrong → reject

### Recommended script
- `scripts/protocol_torture.sh` (15 cases, PASS/FAIL)

---

## Final Checklist

- [ ] `yai kernel ws create testws` creates `~/.yai/run/testws/`
- [ ] `yai kernel ws destroy testws` removes directory
- [ ] Invalid ws_id rejected deterministically
- [ ] Authority checks enforced (arming+role)
- [ ] Root forwards byte-perfect to Kernel
- [ ] All test cases pass

---

## Expected Output (When Complete)

- **Root** = pure router, validates and forwards
- **Kernel** = real workspace lifecycle + enforcement
- **Workspaces** = actual directories with state files
- **Protocol** = hardened end-to-end

---

## Bonus Enhancement

When implementing `ws create`, also create minimal `manifest.json` inside `~/.yai/run/<ws_id>/`:
```json
{
  "ws_id": "testws",
  "created_at": "2026-02-16T10:30:00Z",
  "owner_role": "operator"
}
```
This becomes single source of truth for L2/L3 layers.

---

## Next Steps After v3

Once workspaces are real and hardened:
1. Document socket layout decision in ADR
2. Attach L2 Engine to tenant plane (see next runbook)
3. Add workspace manifest validation