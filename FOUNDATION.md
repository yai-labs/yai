# Foundation

YAI is a sovereign runtime: **authority is explicit**, **execution is deterministic**, and **effects are traceable**. Contracts define behavior; the runtime enforces behavior without silent drift.

## Layer Model

- **L0 — Contracts**: canonical, normative contracts in `deps/yai-law/`
- **L1 — Kernel**: authority enforcement, boundary checks, and policy gating
- **L2 — Engine**: deterministic execution under kernel-governed constraints
- **L3 — Mind / Clients**: proposers and orchestrators that never override authority

## Non-Negotiable Principles

- **Auditability**: externally relevant actions must be reconstructible from logs/events.
- **No silent side effects**: state/effect changes require an explicit command path and policy checks.
- **Effect boundary**: authority boundaries are explicit and enforced at runtime interfaces.
- **Spec-first**: normative behavior is defined in `deps/yai-law`; implementation is never normative.

## Runtime Guarantees

- Authority decisions are made in governed planes, never inferred ad hoc by clients.
- Workspace isolation is first-class and enforced across process/socket boundaries.
- Deterministic components remain deterministic even when integrated with probabilistic clients.

## License

Foundation and governance artifacts in this repository are distributed under Apache-2.0.