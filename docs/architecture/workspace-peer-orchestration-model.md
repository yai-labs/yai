# Workspace Peer Orchestration Model (OP-1)

Status: active  
Owner: runtime  
Effective date: 2026-03-11

## Purpose

Define the owner-side orchestration model for multiple peers attached to one
canonical workspace. The workspace remains the single case boundary and source
of truth; peers are coordinated contributors, not mini-workspaces.

## Core decisions

- One workspace remains canonical for one case, even with many peers.
- Peers are modeled as a workspace peer set (`workspace_peer_membership`).
- Peer membership is distinct from source binding.
- Peer coverage, health, and backlog are explicit per-peer read dimensions.
- Overlap/conflict signals are modeled as baseline states, not yet as full
  conflict-resolution engine.

## Why this model

Secure transport and enrollment alone answer *how peers connect*.
OP-1 answers *how owner coordinates 3+ peers in one case*.

## Canonical concepts

### Workspace peer membership

A persisted owner-side relation that links:

- workspace
- source node
- source binding
- daemon instance
- role/scope/coverage/health/backlog view

This relation is append-only and tracks orchestration state over time.

### Membership vs binding

- `source_binding`: operational attachment for one source flow.
- `workspace_peer_membership`: coordination membership of one peer in the case.

One peer can have multiple bindings; membership stays the workspace anchor.

### Peer health vs workspace health

Peer health states (`ready`, `degraded`, `disconnected`, ...) are local
signals. Workspace viability is evaluated by owner runtime with peer-set
context, not by one peer in isolation.

### Coverage vs authority

Coverage says what a peer observes. It does not grant final workspace
authority, enforcement finality, or evidence finality.

## OP-1 runtime slice

New source-plane class:

- `workspace_peer_membership`

Minimum fields:

- `workspace_peer_membership_id`
- `owner_workspace_id`
- `source_node_id`
- `source_binding_id`
- `daemon_instance_id`
- `peer_role`
- `peer_scope`
- `peer_state`
- `backlog_queued`
- `backlog_retry_due`
- `backlog_failed`
- `coverage_ref`
- `overlap_state`
- `updated_at_epoch`

Population baseline:

- created/updated by owner-side `attach` and `status` handling in source-ingest
- visible in source query summary and tail records
- projected in owner-side graph as membership relation

## Baseline role/scope semantics

Role examples:

- `general`
- `performance`
- `budget`
- `documental`

Scope examples:

- `workspace/default`
- `office/performance`
- `office/documental`

OP-1 keeps role/scope vocabulary open but explicit.

## Baseline coverage/overlap semantics

Coverage reference examples:

- `coverage://workspace/default`
- `coverage://office/performance/kpi`

Overlap states baseline:

- `distinct`
- `overlap_possible`
- `overlap_confirmed`
- `unknown`

These are coordination signals for OP-4, not final conflict decisions.

## What OP-1 does not do

- no full peer registry scheduler yet
- no advanced ordering/dedup engine
- no final conflict resolver
- no peer-to-peer orchestration

## Law coordination note

See `docs/architecture/workspace-peer-orchestration-law-alignment.md`.

OP-1 introduces coordination semantics that law slices can govern in later
waves (membership constraints, coverage overlap expectations, trust posture by
peer role/scope).

## References

- `docs/architecture/source-plane-model.md`
- `docs/architecture/source-plane-read-model.md`
- `docs/architecture/peer-enrollment-and-trust-bootstrap-model.md`
- `docs/program/23-runbooks/workspace-multi-peer-baseline.md`
