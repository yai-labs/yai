# Canonical Data Plane Model (DP-1)

## Purpose
Define the canonical model for YAI data-plane refoundation before backend-specific implementation work.

This document is the DP-1 source of truth for:
- boundary model,
- storage class model (high-level),
- control/data path responsibilities,
- non-negotiable invariants,
- transition guardrails into DP-2..DP-9.

## Scope Boundary

### In scope (DP-1)
- Canonical terminology and model normalization.
- Separation between control-plane decisions and data-plane persistence actions.
- Workspace-scoped persistence boundary definition.
- High-level storage classes and ownership intent.
- Cross-repo responsibility map (`law`, `sdk`, `cli`, `yai`).

### Out of scope (DP-1)
- Concrete backend implementation (DB/graph engines).
- Distributed topology and replication.
- Full operator query surface and data APIs.
- Performance tuning and production HA design.

## Canonical Model

### 1) Execution Cell
The workspace is the data-plane execution cell.  
All persistence operations are resolved inside a workspace boundary and must remain path-jail compliant.

### 2) Authority Mediation
No component writes durable state without governed mediation.  
`Kernel` is the authority gate; `Engine` performs sink operations only after governed dispatch.

### 3) Control/Data Path Separation
Mandatory path:

`cli/sdk -> runtime ingress -> kernel authority -> engine sink -> kernel -> reply`

Data-plane storage is not an independent side-channel and cannot bypass policy/lifecycle gates.

### 4) Storage Class Intent (DP-1 baseline)
DP-1 introduces storage class intent only (detailed role model in DP-2):
- authority state,
- governance/compliance state,
- event/evidence sinks,
- artifact/metadata state,
- transient cognition/graph sink state.

### 5) Runtime Contract Intent
All data-plane actions must expose deterministic outcomes through canonical reply semantics:
- explicit success/failure,
- stable reason code,
- trace/evidence pointer when available.

## Cross-Repo Responsibility Baseline

- `law`: canonical contracts, schemas, policy/lifecycle constraints.
- `yai`: authority mediation and runtime sink orchestration.
- `sdk`: consumer-safe data surface contracts.
- `cli`: operator-safe command surface and readable outcome summaries.

## Non-Negotiable Invariants

1. Workspace scope is mandatory for durable operations.
2. No direct backend write from CLI/SDK.
3. Policy/lifecycle gates cannot be bypassed by data sinks.
4. Deterministic error semantics are required.
5. Traceability links are mandatory for qualification claims.

## DP Transition Rules

### DP-1 -> DP-2
Allowed when storage class names and ownership boundaries are frozen.

### DP-1 -> DP-3
Allowed when topology contract is expressed as canonical layout and migration-ready model.

### DP-1 -> DP-4..DP-9
Allowed only through phase-specific closure checks; no backend shortcuts.

## Verification Hooks (DP-1 baseline)
- model consistency checks between runbook and architecture docs,
- drift checks on key boundary terms (`workspace`, `kernel authority`, `sink mediation`),
- contract anchor presence checks in docs/program.

DP-1 does not claim backend completion; it claims model refoundation readiness for DP-2.
