# Governance

## Scope

This document defines governance for the `yai` runtime implementation repository.
Canonical normative contracts are maintained in `deps/yai-specs`.

## Spec-First Process

For protocol, control, graph, vault, and compliance contract changes:

1. Update contract source in `deps/yai-specs` (or upstream `yai-specs`) first.
2. Review and merge contract change with compatibility/versioning rationale.
3. Update runtime implementation in `yai` to match pinned contracts.
4. Verify with runtime and contract-facing tests before release.

Runtime-first changes that alter normative behavior are not accepted.

## ADR Policy

- ADR location: `docs/architecture/adr/`
- Use template: `docs/templates/ADR.template.md`
- ADR required for:
  - cross-layer boundary changes
  - authority/enforcement model changes
  - wire/protocol behavior changes
  - compatibility policy changes

## Ownership Model

- Specs (`deps/yai-specs/*`) are normative.
- Runtime (`boot/`, `root/`, `kernel/`, `engine/`, `runtime/`) is implementation.
- If implementation conflicts with specs, implementation must be corrected.

## Required Review Areas

Changes in these areas require explicit maintainer review:

- `deps/yai-specs/*` pointer updates in this repo
- `kernel/` enforcement and authority gating
- `engine/` provider gates and execution boundaries
- `runtime/protocol/` envelope/protocol integration

## License

Governance policy files in this repository are licensed under Apache-2.0.
