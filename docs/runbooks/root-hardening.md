---
id: RB-ROOT-HARDENING
title: Root Hardening
status: active
owner: runtime
effective_date: 2026-02-18
revision: 1
supersedes: []
depends_on:
  - RB-WORKSPACES-LIFECYCLE (optional, if already exists)
adr_refs:
  - docs/design/adr/ADR-002-root-entrypoint.md
  - docs/design/adr/ADR-006-unified-rpc.md
  - docs/design/adr/ADR-008-connection-lifecycle.md
decisions:
  - docs/design/adr/ADR-002-root-entrypoint.md
  - docs/design/adr/ADR-006-unified-rpc.md
  - docs/design/adr/ADR-008-connection-lifecycle.md
related:
  adr:
    - docs/design/adr/ADR-002-root-entrypoint.md
    - docs/design/adr/ADR-006-unified-rpc.md
    - docs/design/adr/ADR-008-connection-lifecycle.md
  specs:
    - deps/yai-specs/specs/protocol/include/transport.h
    - deps/yai-specs/specs/protocol/include/auth.h
    - deps/yai-specs/specs/protocol/include/errors.h
    - deps/yai-specs/specs/protocol/include/yai_protocol_ids.h
  test_plans:
    - docs/test-plans/hardfail.md
  tools:
    - tools/bin/yai-verify
    - tools/bin/yai-gate
    - tools/bin/yai-suite
tags:
  - runtime
  - hardening
---

# RB-ROOT-HARDENING — Root ↔ Kernel Boundary Hardening (YAI 0.1.x)

This is an operational runbook for **YAI 0.1.x**.

Objective: harden the **Root control plane** as a deterministic, auditable, envelope-only boundary between clients and the Kernel.

Root must behave like a governed cable:
- validates envelope invariants
- enforces handshake + basic policy
- forwards bytes without mutation
- never "silent drops"
- logs everything in an indestructible way

This runbook does NOT redesign architecture. It strengthens enforcement and observability without changing the planes model.

## 1) Purpose

Harden the Root control plane as a deterministic, auditable, envelope-only boundary between clients and the Kernel.

## 2) Preconditions

- [ ] `deps/yai-specs` protocol headers are present and treated as source-of-truth.
- [ ] Kernel boots and accepts control connections.
- [ ] A baseline end-to-end ping command is already green.

## 3) Inputs

- Protocol anchors:
  - `deps/yai-specs/specs/protocol/include/transport.h`
  - `deps/yai-specs/specs/protocol/include/auth.h`
  - `deps/yai-specs/specs/protocol/include/errors.h`
- Tooling:
  - `tools/bin/yai-verify`
  - `tools/bin/yai-gate`
  - `tools/bin/yai-suite`

## 4) Procedure

### Position in the global sequence

1. Root hardening ✅ (this document)
2. Workspace lifecycle
3. Engine attach
4. Data plane
5. Mind Redis STM

### Hard prerequisites (must be true before starting)

- `deps/yai-specs` headers are present and treated as source-of-truth
- Kernel can boot and accept control connections
- A baseline "ping" command exists end-to-end (CLI → Root → Kernel → response)

If any prerequisite is not true: stop and fix baseline first.

---

### Scope

### In scope

- Strict envelope validation in Root (mechanical guardrails)
- Mandatory handshake gate (only allow handshake before session is established)
- Byte-perfect forward/relay (Root does not mutate envelope nor payload)
- Deterministic error reply (always a response frame, never silent drop)
- Indestructible Root logging (file + stderr)

### Out of scope

- New protocol fields (no envelope redesign)
- New business logic in Root (no payload interpretation)
- Engine/Mind changes beyond what is required for tests
- Data plane / persistence

---

### Operational Workflow (Daily)

### Clean runtime before each test round

```bash
pkill -f yai-root-server || true
pkill -f yai-kernel || true
pkill -f yai-boot || true
rm -rf ~/.yai/run/root/root.log || true
```

<a id="phase-root-boot-baseline"></a>
### Build + boot baseline

```bash
make clean
make
yai-boot --master
```

### Sanity check

```bash
yai root ping
```

Expected:

- `pong ok` (or equivalent)
- `~/.yai/run/root/root.log` exists and is appending

---

### Deliverables (Phased)

This runbook is delivered through sub-phases under YAI 0.1.x.
Each phase must compile, run, and be verifiable before moving on.

---

<a id="phase-0-1-0-protocol-guardrails"></a>
### 0.1.0 — Protocol Guardrails (no business logic)

**Branch:** `feat/root-hardening-0.1.0-guardrails`  
**Goal:** Root and Kernel share identical mechanical wire rules and error codes.
**Milestone Pack:** `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`

#### File targets

READ FIRST:

- `deps/yai-specs/specs/protocol/include/transport.h`
- `deps/yai-specs/specs/protocol/include/yai_protocol_ids.h`
- `deps/yai-specs/specs/protocol/include/errors.h`
- `deps/yai-specs/specs/protocol/include/auth.h`

CODE (likely):

- `root/src/yai_root_server.c` (or current Root server file)
- `kernel/src/core/rpc_binary.c` (or equivalent decode/dispatch point)

#### Guardrails required (mechanical invariants)

- `magic` must match
- `version` must be supported
- `payload_len <= YAI_MAX_PAYLOAD`
- `arming ∈ {0,1}`
- `role` must be within known enum range
- `ws_id` validation: reject invalid patterns deterministically
- checksum policy (0.1.x):
  - `checksum == 0` is mandatory
  - non-zero checksum is a deterministic reject

#### Standard error codes (numeric)

Root and Kernel must reject using the same codes from specs (no local enums).
Minimum set expected:

- bad magic
- bad version
- bad ws_id
- need handshake
- arming required
- role required
- payload too big
- bad checksum

#### Verification

- `yai root ping` still works
- invalid envelope inputs get an error reply (not silent close)

#### Acceptance (0.1.0)

- [ ] Root and Kernel reject invalid frames with the same numeric codes
- [ ] No silent drop on malformed inputs
- [ ] Build passes and baseline boot still works

---

<a id="phase-0-1-1-byte-perfect-router"></a>
### 0.1.1 — Root = Byte-Perfect Router (forward/relay)

**Branch:** `feat/root-hardening-0.1.1-router`  
**Goal:** Root becomes a pure router with deterministic rejects + indestructible logging.
**Milestone Pack:** `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.1.md`

#### File targets

Root:

- `root/src/yai_root_server.c`
- `root/src/control_transport.c` (if exists)

Kernel side used for comparison:

- `kernel/src/core/transport.c`
- `kernel/src/core/control_transport.c` (if exists)

Specs:

- `deps/yai-specs/specs/protocol/include/transport.h`

#### Rules

Root MUST NOT:

- interpret payload
- change envelope fields
- regenerate trace_id
- rewrite ws_id

Root MUST:

- validate envelope invariants BEFORE forward
- forward envelope + payload bytes as received
- relay response bytes as received
- on reject: always send an error frame, then close

#### Deterministic error reply policy (0.1.x)

- Response must always be a valid response frame
- payload may be minimal JSON (optional), but error code MUST be in envelope-level error
- do not "timeout as error"

#### Indestructible logging

- path: `~/.yai/run/root/root.log`
- create directory if missing: `~/.yai/run/root`
- open with append, never truncate
- log minimal fields per line:
  - timestamp
  - ws_id (or "system" if not available)
  - trace_id
  - command_id
  - decision (FORWARD/REJECT)
  - error_code (if reject)

#### Verification

Re-run baseline:

- `yai root ping`

Protocol negative tests (at least):

- wrong magic
- wrong version
- payload too big

#### Acceptance (0.1.1)

- [ ] Root behaves as a "smart cable": validate + forward + relay
- [ ] Every reject returns a response frame (no silent close)
- [ ] `root.log` always exists and appends

---

<a id="phase-0-1-2-envelope-authority-gate"></a>
### 0.1.2 — Envelope-Only Authority Gate (Root + Kernel)

**Branch:** `feat/root-hardening-0.1.2-authority-gate`  
**Goal:** privileged commands require arming+role, enforced in Root and Kernel (defense-in-depth).
**Milestone Pack:** `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.2.md`

#### File targets

Specs:

- `deps/yai-specs/specs/protocol/include/auth.h`
- `deps/yai-specs/specs/protocol/include/roles.h` (if present)

Root:

- `root/src/yai_root_server.c`

Kernel:

- `kernel/src/core/rpc_binary.c`
- `kernel/src/core/yai_session.c` (or current authority/session point)

#### Policy

Authority is decided ONLY by envelope metadata:

- command_id
- arming
- role
- ws_id presence/validity

Never read payload to make authority decisions.

Implement a single mapping (table or switch) shared conceptually between root+kernel.
Minimum expectation:

- destructive or governance commands require:
  - `arming=1`
  - `role>=operator`

#### Verification

- Try privileged command without arming → deterministic reject
- Try privileged command with arming but low role → deterministic reject

#### Acceptance (0.1.2)

- [ ] Root rejects privileged commands early (fast fail)
- [ ] Kernel rejects again (defense-in-depth)
- [ ] Error codes identical in both paths

---

### 0.1.3 — ws_id Validation Centralization (single definition)

**Branch:** `feat/root-hardening-0.1.3-ws-id-single-source`  
**Goal:** one ws_id validator used everywhere (Root/Kernel/CLI), eliminating drift.
**Milestone Pack:** `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.3.md`

#### File targets

Specs (single source of truth):

- `deps/yai-specs/specs/protocol/include/transport.h` (static inline validator)

Consumers:

- Root server file(s)
- Kernel decode/dispatch file(s)
- CLI client path (where envelope is formed)

#### ws_id rule (0.1.x)

- length: 1..35
- charset: `[A-Za-z0-9_-]`
- forbidden: `/`, `~`, whitespace
- optional: forbid leading `.`

#### Verification

- invalid ws_id never reaches dispatch
- CLI cannot send invalid ws_id (client-side guard) AND server rejects anyway

#### Acceptance (0.1.3)

- [ ] No divergent validators remain in repo
- [ ] "ws_id as path" class of bugs cannot reappear

---

### 0.1.4 — Kernel Hard Reject on Invalid ws_id (zero side effects)

**Branch:** `feat/root-hardening-0.1.4-kernel-hard-reject`  
**Goal:** Kernel must not create sessions/dirs for invalid ws_id; must respond deterministically.
**Milestone Pack:** `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.4.md`

#### File targets

- `kernel/src/core/yai_session.c`
- `kernel/src/core/rpc_binary.c`
- `kernel/src/core/rpc_codec.c` (if present)

Specs:

- `deps/yai-specs/specs/protocol/include/transport.h`

#### Rules

If ws_id invalid/empty/overflow:

- no session creation
- no filesystem effects
- deterministic error response frame

Avoid C bug:

- `env->ws_id` is an array, check `env->ws_id[0] == '\0'`

#### Verification

Send invalid ws_id:

- assert `~/.yai/run/<ws_id>` does NOT appear
- assert error frame is returned

#### Acceptance (0.1.4)

- [ ] Kernel has zero side effects on invalid ws_id
- [ ] Kernel always responds deterministically with error frame

---

### 0.1.5 — Test Matrix + Torture Suite

**Branch:** `feat/root-hardening-0.1.5-torture`  
**Goal:** repeatable torture tests that prove hardening is real.
**Milestone Pack:** `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.5.md`

#### Minimum test cases

1. handshake ok
2. handshake wrong version → error + close
3. ping valid ws
4. ping invalid ws (`/`, `~`, space) → deterministic reject
5. ws overflow length (36+)
6. missing ws_id (empty string) → reject
7. arming violation (privileged cmd without arming)
8. role violation (arming ok but role low)
9. payload_len > max
10. wrong magic
11. wrong version
12. checksum != 0 (policy "0 only" in 0.1.x)

#### Tools

In 0.1.x we move tests under tools. If `tools/` still exists, it is temporary.

Preferred:

- `tools/python/yai_tools/protocol/handshake_test.py`
- `tools/bin/yai-suite` or `tools/bin/yai-gate`

Temporary compatibility:

- `tests/integration/test_handshake.py`
- `tools/dev/protocol_tester`

#### Verification commands (must be runnable)

- `yai root ping`
- `yai verify core` (or equivalent)
- `yai gate ws` (or equivalent)
- torture runner (single command) that prints PASS/FAIL per case

#### Acceptance (0.1.5)

- [ ] all cases pass deterministically, in sequence
- [ ] every fail is auditable in `root.log` + kernel logs

---

## 5) Verification

Mandatory log location:

- Root: `~/.yai/run/root/root.log`

Every reject must produce:

- a response frame (error code)
- a log line containing at least:
  - trace_id
  - ws_id (or "system")
  - command_id
  - error_code

---

## 6) Failure Modes

- Symptom: root/kernal behavior diverges on malformed envelope handling.
  - Fix: realign reject codes and response framing before phase closure.
- Symptom: silent drops appear in negative-path tests.
  - Fix: force deterministic error frames and rerun hardfail vectors.
- Symptom: logging evidence is incomplete for rejects.
  - Fix: restore append-only root log path and attach CI/runtime artifacts.

## 7) Rollback

Rollback must be clean:

- each phase is isolated to a branch
- merge only after acceptance passes
- do not keep partial enforcement in main

If a phase causes regressions:

- revert that phase (single merge commit / or squash revert)
- never "hotfix drift" inside a later phase

---

## 8) References

### Upstream proposals

- `docs/design/proposals/PRP-001-runtime-topology-and-authority.md`
- `docs/design/proposals/PRP-002-unified-rpc-and-cli-contract.md`
- `docs/design/proposals/PRP-004-contract-baseline-lock-and-pin-policy.md`
- `docs/design/proposals/PRP-005-formal-coverage-roadmap.md`

### Milestone packs

- `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`
- `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.1.md`
- `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.2.md`
- `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.3.md`
- `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.4.md`
- `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.5.md`

## 9) Final Definition of Done

- [ ] Root validates invariants + handshake gate
- [ ] Root is byte-perfect forward/relay
- [ ] Root never silent drops (always responds)
- [ ] Root logs are indestructible + informative
- [ ] Kernel rejects invalid ws_id with zero side effects
- [ ] authority gating enforced in Root + Kernel
- [ ] torture suite passes and is repeatable

## Traceability

- ADR refs:
  - `docs/design/adr/ADR-002-root-entrypoint.md`
  - `docs/design/adr/ADR-006-unified-rpc.md`
  - `docs/design/adr/ADR-008-connection-lifecycle.md`
- Law/spec refs:
  - `deps/yai-specs/specs/protocol/include/transport.h`
  - `deps/yai-specs/specs/protocol/include/auth.h`
  - `deps/yai-specs/specs/protocol/include/errors.h`
- MPs:
  - `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`
  - `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.1.md`
  - `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.2.md`
  - `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.3.md`
  - `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.4.md`
  - `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.5.md`
