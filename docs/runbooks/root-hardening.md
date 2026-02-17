---
id: RB-ROOT-HARDENING
status: active
effective_date: 2026-02-16
revision: 3
supersedes: []
law_refs:
  - deps/yai-specs/protocol/transport.h
  - deps/yai-specs/protocol/auth.h
owner: runtime
---


# YAI Root Hardening v2 — Operational Runbook

**Branch:** `feat/root-hardening-v2`  
**Objective:** Harden L0 Root ↔ L1 Kernel boundary with deterministic protocol rules, envelope-only enforcement, byte-perfect routing, and indestructible logging/audit.

---

## Daily Prep

### 1. Create branch
```bash
git checkout -b feat/root-hardening-v2
```

### 2. Clean runtime (before each test round)
```bash
pkill -f yai-root-server || true
pkill -f yai-kernel || true
pkill -f yai-boot || true
rm -rf ~/.yai/run/root/root.log || true
```

### 3. Build + boot baseline
```bash
make clean
make
yai-boot --master
```

### 4. Quick sanity check
```bash
yai root ping
```
**Expected:** pong ok + `root.log` created and appending correctly

---

## STEP 0: Protocol Guardrails (Zero Business Logic)

### Files to read FIRST
- `deps/yai-specs/protocol/transport.h`
- `deps/yai-specs/protocol/yai_protocol_ids.h`
- `deps/yai-specs/protocol/errors.h` *(recommended: separate from auth)*

### Goal
Define mechanical wire-format rules identical across: root / kernel / CLI / engine

### Minimum deliverables

**A) Standard error codes (numeric)**
- `YAI_E_BAD_MAGIC`
- `YAI_E_BAD_VERSION`
- `YAI_E_BAD_WS_ID`
- `YAI_E_NEED_HANDSHAKE`
- `YAI_E_ARMING_REQUIRED`
- `YAI_E_ROLE_REQUIRED`
- `YAI_E_PAYLOAD_TOO_BIG`
- `YAI_E_BAD_CHECKSUM`

**B) Explicit frame invariants**
- `payload_len <= YAI_MAX_PAYLOAD`
- `arming ∈ {0,1}`
- `role` within range (guest/user/operator/sovereign) → out-of-range = reject
- `ws_id` null-terminated sender-side, validated receiver-side

**C) Checksum policy** (even if not implementing CRC tomorrow)
- For now: `checksum == 0` mandatory
- If `checksum != 0` → deterministic reject with `YAI_E_BAD_CHECKSUM`

### Done when
- Root and Kernel can "reject-with-error-frame" using same codes
- CLI/engine can interpret errors uniformly

---

## STEP 1: forward_to_kernel in Root (Pure Router)

### Files to read FIRST
- `kernel/src/bin/yai_root_server.c`
- `kernel/src/core/control_transport.c`
- `kernel/src/core/transport.c`
- `deps/yai-specs/protocol/transport.h` *(source-of-truth envelope)*

### Goal
Root becomes pure router:
- Does not interpret payload
- Does not mutate envelope
- Byte-perfect forward to kernel and byte-perfect relay to client

### Protocol robustness included

**A) Root must never "silent drop"**  
Every reject → responds with:
- Valid envelope
- Short JSON payload: `{ "ok": false, "code": <ERR>, "msg": "..." }`
- Then closes

**B) Hard validation BEFORE forward**
- `magic/version`
- `payload_len` bounds
- `ws_id` validate (STEP 3; stub local match for now)
- Handshake gate: if not done, only pass `YAI_CMD_HANDSHAKE`

**C) Byte-perfect forward/relay**
- Forward identical `env` + identical payload to kernel
- Read response (envelope+payload) and relay identical to client
- Do not regenerate `trace_id`
- Do not touch `ws_id`

### Root "indestructible" logging (definitive reintroduction)
- File: `~/.yai/run/root/root.log`
- Open with `O_CREAT | O_APPEND`
- If directory missing: `mkdir -p ~/.yai/run/root`
- Log to file + stderr (stderr live, file audit)

### Done when
- Root behaves as "smart cable": validates and forwards
- Every error produces deterministic response (not timeout/silent close)
- `root.log` always created and appending

---

## STEP 2: Authority Check Envelope-Only (Defense-in-Depth)

### Files to read FIRST
- `deps/yai-specs/protocol/auth.h`
- `kernel/src/core/rpc_binary.c`
- `kernel/src/core/yai_session.c`

### Goal
Authority enforcement on envelope metadata only. Never on payload.

### Practical deliverables

**A) Policy: command → required role/arming (table)**
- Static array (C) or clear switch mapping
- Example:
  - `ws_create/ws_destroy/stop/...` require:
    - `arming=1`
    - `role>=operator`

**B) Double enforcement**
- Root applies policy before forward (fast reject)
- Kernel applies same policy (defense-in-depth)

**C) Deterministic reject**
- Error frame with code:
  - `YAI_E_ARMING_REQUIRED` / `YAI_E_ROLE_REQUIRED`

### Done when
- Privileged command never passes without correct arming+role
- Rejects identical whether from root or kernel

---

## STEP 3: Centralize `is_valid_ws_id` (SINGLE Definition)

### Target file
- `deps/yai-specs/protocol/transport.h` (static inline)

### Goal
Single point of truth for ws_id validation

### ws_id rule (recommended)
- Length: 1..35
- Charset: `[A-Za-z0-9_-]`
- Forbidden: `/`, `~`, spaces
- (Optional) forbid initial `.`

### Must be used by
- root
- kernel
- CLI
- (later) engine

### Done when
- No module has divergent manual regex/validations
- Bug "ws_id = path" can never reappear

---

## STEP 4: Kernel Hard Reject on Invalid ws_id (Zero Side Effects)

### Files to read FIRST
- `kernel/src/core/yai_session.c`
- `kernel/src/core/rpc_binary.c`
- `kernel/src/core/rpc_codec.c` (if present)
- `deps/yai-specs/protocol/transport.h`

### Goal
No dispatch if:
- Invalid ws_id
- Empty ws_id
- Overflow length

Fail fast, deterministic log, error response.

### Robustness included
- Kernel must respond with error frame `YAI_E_BAD_WS_ID` (not just break/close)
- Kernel does not create sessions/dirs if ws_id invalid (zero side effects)

### Important warning fix
In C, `env->ws_id` is not a pointer (it's an array), so:
```c
if (!env->ws_id || strlen(env->ws_id) == 0)
```
Must change to:
```c
if (env->ws_id[0] == '\0')
```

### Done when
- Attempts with invalid ws_id create nothing on disk
- Kernel always responds with deterministic error

---

## STEP 5: Test Matrix + Protocol Torture

### Mandatory tests (minimum)
1. Handshake ok
2. Handshake wrong version → error code + close
3. Ping valid ws
4. Ping invalid ws (slash/tilde/space) → deterministic reject
5. ws with illegal characters
6. ws overflow length (36+)
7. Arming violation (ws_destroy without arming)
8. Missing ws_id (ws_id = "" / simulations)
9. payload_len > max
10. Wrong magic
11. Wrong version
12. checksum != 0 (while policy is "zero only")

### Test tools (to use)
- `scripts/test_handshake.py` (add cases: wrong version/magic/len)
- `./tools/cli/yai root ping`
- `./tools/cli/yai kernel ws create <id>`
- `./tools/cli/yai kernel ws destroy <id> --arming --role operator`
- (New recommended) `scripts/protocol_torture.sh` (15 PASS/FAIL cases)

### Done when
- All tests repeatable and pass in sequence
- Every FAIL is "auditable" (log + consistent error)

---

## Note: "ws create ok but directory doesn't exist"

If root responds `{ "status": "ok" }` but `~/.yai/run/<ws>` doesn't appear, typical of "root stub": not forwarding or kernel not creating session/paths.

With STEP 1 (forward) + STEP 4 (kernel creates only on valid ws), path `~/.yai/run/testws` must appear.

---

## Perfect Daily Sequence (Sure Shot)

1. STEP 0 (errors + invariants)
2. STEP 1 (forward + relay + log + reject-with-error)
3. STEP 3 (ws_id inline)
4. STEP 2 (authority gate root+kernel)
5. STEP 4 (kernel hard reject + session create)
6. STEP 5 (torture)

---

## Final Checklist (Before Closing)

- [ ] `yai root ping` ok
- [ ] `yai kernel ws create testws` actually creates `~/.yai/run/testws`
- [ ] `yai kernel ws destroy testws` requires arming+operator (else reject)
- [ ] `root.log` always present, appending, contains RECV + decision + error codes

---

## Expected Output (When Hardened)

- **Root** = pure router + envelope-only policy + error replies + logging
- **Kernel** = envelope-only enforcement + ws_id hard reject + zero side effects on dirty input
- **CLI** = can no longer send "path-like" ws_id (by definition)
- **Protocol** = unified invariants tested with torture script