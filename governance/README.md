# Governance Root (Canonical Destination)

This directory is the canonical governance root of the unified YAI repository.

Scope:

- authority model and sovereignty boundaries
- governance operational content: grammar, registry, classification
- domains, control families, specializations
- overlays, compliance layers, manifests, contracts, schemas
- governance ingestion and runtime-facing policy artifacts

Authority vs Foundation vs Formal:

- `authority/`: operational governance (roles, scopes, decisions, publication,
  deprecation, compatibility, status)
- `../foundation/`: conceptual system basis (axioms, invariants, boundaries,
  extensions, terminology) at root level
- `../formal/`: formal verification/model-checking artifacts at root level

Semantic spine:

- `grammar/`: governance language rules, semantics, and grammar schemas
- `registry/`: canonical primitives, governable objects, commands, ids, and
  registry schemas
- `domains/index/domain-model.matrix.v1.json`: index-driven family/domain/
  specialization resolution model used by governance runtime loading

Compliance and overlays:

- `compliance/`: normative compliance modules and applicability metadata
- `overlays/`: regulatory/sector/contextual overlay descriptors + matrices
- `packs/`: versioned/materialized governance bundles

Migration policy:

- `governance/` is the final destination for governance content that currently
  exists in external or embedded forms.
- `transitional/embedded-*` is used only to track migration markers and
  temporary compatibility references during convergence.

Roadmap anchor:

- Block A (A1-A19): refound `yai` as the unified repository target.
- Block B (B1-B13): absorb external governance content and sunset split-repository topology.

Reference:

- `docs/architecture/repository/governance-placeholder-spine-a6.md`
- `transitional/legacy-maps/b1-governance-content-import-map.md`
