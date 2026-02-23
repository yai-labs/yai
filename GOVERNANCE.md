# Governance

## Scope

This document defines governance for the **`yai` runtime implementation repository**.

- **Normative contracts** live in `deps/yai-specs/` (canonical source of truth).
- This repository implements those contracts across runtime planes:
  `boot/`, `root/`, `kernel/`, `engine/`, `runtime/`, and `mind/` (L3 module).

If implementation and contracts disagree, **implementation is wrong** and must be corrected (or the contract must be explicitly revised upstream).

## Spec-First Process

For any change that affects contract-facing behavior (protocol, control plane, graph, vault, compliance surfaces):

1. **Change contracts first** in `deps/yai-specs/` (or upstream `yai-specs`) and document:
   - rationale
   - compatibility impact
   - versioning decision
2. **Review and merge** the contract change.
3. **Update runtime implementation** in `yai` to match the pinned contract version.
4. **Verify** via CI gates + contract-facing tests before merge/release.

Runtime-first changes that alter normative behavior are **not accepted**.

## Governance Gates

The repository is operated as a gated runtime:

- **Pin discipline**: contract pointers are treated as release-critical inputs.
- **Changelog discipline**: contract-facing changes require explicit Unreleased entries.
- **Proof/trace discipline**: evidence artifacts must support hardening phases and releases where applicable.

(See `tools/` and CI workflows for enforcement.)

## ADR Policy

- ADR location: `docs/20-governance/design/adr/`
- Template: `docs/20-governance/design/adr/ADR-000-template.md`

ADR is required for:

- cross-layer boundary changes (L0–L3 interactions)
- authority/enforcement model changes
- wire/protocol behavior changes
- workspace/session lifecycle changes
- compatibility/versioning policy changes
- release/distribution policy changes (bundles, pins, verification rules)

## Ownership Model

- `deps/yai-specs/*` is **normative**.
- `boot/`, `root/`, `kernel/`, `engine/`, `runtime/`, `mind/` are **implementation**.
- `tools/` provides **verification and release gates**.
- `docs/` is operational/architecture guidance; it must not redefine contracts.

If implementation conflicts with specs, implementation must be corrected.
If a spec is wrong, it must be fixed in `yai-specs` and pinned forward explicitly.

## Required Review Areas

Changes in these areas require explicit maintainer review:

- Contract pin updates: `deps/yai-specs/*` pointers and related pin tooling
- `kernel/` enforcement, authority gating, session/workspace boundaries
- `engine/` provider gates, execution boundaries, external effect control
- `runtime/protocol/` envelope/protocol behavior and compatibility surfaces
- `mind/` any change that affects contract-facing interaction with L1/L2

## License

Governance policy files in this repository are licensed under Apache-2.0.