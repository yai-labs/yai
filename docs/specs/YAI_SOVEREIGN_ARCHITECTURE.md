# YAI Sovereign Architecture (Blueprint)

## A) Executive Summary
YAI is a governed stack where **Law defines authority**, and **runtime implements enforcement**. The system is decomposed into Law, Kernel, Engine, Mind, and Control Plane, each with explicit contracts and non-overlapping responsibilities.

This document is an implementation blueprint. It defines boundaries, contracts, acceptance criteria, and stop conditions so the system can evolve without drift, feature creep, or loss of observability.

---

## B) Non‑Negotiable Principles
1. **Authority is explicit**: Law is authoritative; Kernel enforces; Mind cannot override.
2. **Determinism first**: Engine and Kernel must be deterministic and auditable.
3. **Observability is mandatory**: Every decision produces an audit event.
4. **Isolation of intelligence**: ML is never in the Kernel authority plane.
5. **Reproducibility**: Runtime state must be reconstructible from durable state and events.

---

## C) Components and Boundaries

### Law (Authority)
- Defines axioms, invariants, boundaries, and formal models.
- Canonical specifications live under `law/` and are the source of truth.

### Kernel (L1 Authority Enforcement)
- Enforces the canonical state machine and authority gating.
- No ML. Only explainable statistical checks if needed.
- Produces deterministic state transitions and trace evidence.

### Engine (L2 Deterministic Execution)
- Executes deterministic tasks and resource operations.
- No authority changes. Operates under Kernel policy.

### Mind (L3 Orchestration)
- Cognitive orchestration and planning.
- Consumes events and proposes actions, but does not enforce.

### Control Plane (Daemon/RPC/Events)
- Authoritative runtime supervisor per workspace.
- Owns lifecycle, process supervision, RPC surface, and event stream.
- Multiple clients (CLI/TUI/GUI) attach to the same workspace.

---

## D) Contracts

### D1) RPC v1 (UDS per‑workspace)
**Transport**: Unix Domain Socket (per WS)

**Path**: `~/.yai/run/<ws>/control.sock`

**Versioning**: `rpc_version = 1` (implicit, documented in `RPC_V1.md`)

**Error model**:
- Errors are structured responses with `type=error` and `message`.
- Transport errors are treated as “daemon unavailable”.

**Requests (v1)**:
- `ping`
- `status`
- `up` {build, no_engine, no_mind, ai, timeout_ms}
- `down` {force, shutdown}
- `providers_discover|list|pair|attach|detach|status`
- `events_subscribe`

**Responses (v1)**:
- `pong`
- `status` {state, alive, daemon_pid}
- `up_ok` / `down_ok`
- `providers` / `provider_status`
- `events_started`
- `error` {message}

See `docs/specs/RPC_V1.md` for schema details.

### D2) Events Stream (NDJSON)
**Transport**: RPC subscribe on `control.sock`.

**Format**: 1 JSON object per line. Required fields:
- `ts` (epoch seconds)
- `kind` (string)
- `data` (object)

**Minimum events**:
- `daemon_started`
- `ws_up_started`
- `ws_up_complete`
- `proc_started` {proc, pid, pgid}
- `proc_exit` {proc, code?, signal?}
- `status_changed` {boot, kernel, engine, mind}
- `kernel_dead`
- `ws_down_started`
- `ws_down_complete`
- `provider_attached` / `provider_detached`

See `docs/specs/EVENTS_V1.md`.

### D3) Provider Trust Lifecycle
**Discover (untrusted)**
- Lists candidates without storing trust.

**Pair (trusted)**
- Creates trusted entry in `~/.yai/trust/providers.json`.

**Attach (active)**
- Sets active provider per workspace at `~/.yai/run/<ws>/provider.json`.

**Detach**
- Clears active provider for workspace.

See `docs/specs/PROVIDERS_TRUST.md`.

---

## E) Measurable Objectives (Acceptance Criteria)
1. **Hard‑fail kernel**: if kernel dies, runtime is marked down and sockets are cleaned.
2. **Multi‑client**: 2+ event subscribers receive the same event stream concurrently.
3. **Idempotent down**: two consecutive `down` calls are safe and consistent.
4. **Per‑WS isolation**: socket, lock, pidfiles are namespaced by workspace.
5. **Auditability**: every lifecycle transition emits an event.

---

## F) Stack Choices (Pragmatic, Minimal)
- **Graph**: `petgraph` (stable, mature, Rust‑native).
- **Vector ANN**: `usearch` (lightweight, fast, local).
- **Inference**: `candle` (preferred for small local inference), `burn` if training needed later.
- **Kernel**: C only; no ML.

**Rationale**: small dependencies, local‑first, explainable, minimal runtime risk.

---

## G) Where Models Come From (Documented Only)
**Embeddings (v1)**
- Default: small sentence‑transformers.
- Candidates:
  - `all-MiniLM-L6-v2` (lightweight, common)
  - `bge-small-en-v1.5` / `bge-small-it` (quality)
- Constraint: local CPU, acceptable latency.
- Formats: ONNX or safetensors.

**Ranking/Retrieval**
- Start with embeddings + ANN, no LLM required.

**Engine Forecast (Phase 2)**
- First: deterministic EWMA / peak detect.
- Later: tiny classifier (low/med/high) if needed.

**Kernel Sentry**
- No ML. Only explainable statistics (z‑score/MAD/rate‑limit).

---

## H) Roadmap (Value vs Risk)

### Phase 0 — Low Risk / High Integrity
- Events stream + kernel hard‑fail + verify scripts
- Definition of Done:
  - Hard‑fail cleanup verified
  - Multi‑client events verified

### Phase 1 — MindGraph + Retrieval
- Graph construction + vector retrieval
- Definition of Done:
  - Deterministic graph replay
  - Retrieval metrics logged

### Phase 2 — Engine Elasticity
- Deterministic baseline (EWMA/peak)
- Definition of Done:
  - No ML in Kernel
  - Forecast is advisory only

### Phase 3 — Kernel Sentry
- Statistical policy guardrails only
- Definition of Done:
  - Policy versioned
  - No ML in Kernel plane

---

## I) Manual Test Suite (Commands + Expected)
Runbook: `docs/runbooks/TEST_EVENTS.md`
Runbook: `docs/runbooks/TEST_HARDFAIL.md`
Runbook: `docs/runbooks/TEST_GRAPH.md`
Runbook: `docs/runbooks/TEST_EMBED.md`

1) **Up and status**
```bash
yai up --ws dev --detach
yai status --ws dev
```
Expected: socket exists, kernel alive, status coherent.

2) **Events**
```bash
yai events --ws dev
```
Expected: events for lifecycle + processes.

3) **Hard‑fail**
```bash
kill -TERM <kernel_pid>
```
Expected: `kernel_dead` event, ws down, socket removed.

4) **Idempotent down**
```bash
yai down --ws dev
yai down --ws dev
```
Expected: no errors, clean down.

---

## J) Anti‑Drift (What NOT to Do)
- Do not add ML into Kernel authority plane.
- Do not allow decision paths without events.
- Do not multiplex multiple workspaces on one socket.
- Do not bypass Law for runtime policy changes.
- Do not add features that reduce observability.

---

## Stop Criteria
- If a feature reduces observability or makes state non‑reproducible → **STOP**.
- If ML enters Kernel authority plane → **STOP**.
- If automatic decisions lack audit events → **STOP**.
