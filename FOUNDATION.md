# Foundation

YAI is a governed runtime system where authority is explicit, execution is deterministic,
and externally relevant effects are traceable.

This repository is the single canonical system root.

## Layer model

- **L0 Governance**: contracts, schemas, manifests, registry, and policy content in `governance/`
- **L1 Kernel**: authority boundaries and enforcement gates
- **L2 Engine**: deterministic execution under governance constraints
- **L3 Agents/Clients**: consumers of governed services, never authority owners

## Non-negotiable principles

- **Single source of truth**: runtime, governance, docs, tests, and tooling converge in one repo
- **No silent side effects**: effectful transitions require explicit command and policy path
- **Auditability**: decisions and evidence must be reconstructible from persistent records
- **Bounded authority**: authority scope is explicit and enforced at runtime boundaries
- **Spec-first implementation**: governance artifacts are normative; code must conform

## Canonical topology

- `cmd/`
- `lib/`
- `include/`
- `governance/`
- `foundation/`
- `formal/`
- `docs/`
- `tests/`
- `tools/`
- `data/`

`transitional/` is migration-only and not part of canonical 1.0.0 topology.

## License

Apache-2.0.
