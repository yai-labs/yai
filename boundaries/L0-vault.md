# L0 — Vault Boundary

## Purpose

Define the physical boundary of the YAI Vault as the single canonical contract for:

- shared memory layout (header + regions + alignment rules)
- field offsets and packing constraints
- command and state ID registries used across system components

If this boundary drifts, the system does not share a common execution surface and is no longer a coherent instance of YAI.

---

## Authority

L0 is the single authority for:

- the Vault memory layout and its invariants
- the canonical ID registry for commands and states
- naming rules for Vault channels and regions

No downstream component may redefine offsets, IDs, or header semantics.

---

## Definitions

**Vault**
A bounded, shared-memory execution surface used for inter-component coordination.

**Vault Channel / Region**
A named memory region with fixed layout rules and a stable semantic role.

**ID Registry**
A canonical mapping for command IDs and state IDs used in cross-language interfaces.

---

## In Scope

- Shared memory layout and field offsets
- Header layout, packing/alignment rules, and versioning
- Command and state ID values (registry)
- Naming rules for channels / regions
- Compatibility rules (what counts as breaking vs compatible change)

---

## Out of Scope

- Execution logic, scheduling, planning
- Policy, intent, or governance workflows
- UI, tooling, or build systems
- Higher-level protocol semantics beyond ID + wire/header constants

---

## Canonical Contract

### Vault Header (Minimum Required Fields)

The Vault header MUST include, at minimum:

- magic (constant signature)
- version_major, version_minor
- layout_id (identifier for layout family)
- total_size
- region_table_offset (or equivalent fixed mapping)
- flags (reserved, forward-compatible)
- checksum or integrity field (optional but recommended)

The header MUST be:

- explicitly packed/aligned
- stable across languages (C/Rust)
- validated at kernel startup

---

### Layout and Packing Rules

- All offsets are defined in bytes, relative to Vault base.
- All structs shared across languages MUST define explicit alignment rules.
- No implicit padding reliance is allowed across compilers.
- A layout change that modifies any shared offset is a breaking change unless versioned and gated.

---

### Command and State ID Registry

- All command IDs and state IDs MUST be defined in a single registry surface (header/constants).
- IDs MUST be stable, unique, and non-reused.
- ID ranges MAY be reserved (e.g., core vs experimental), but the reservation itself is L0-authoritative.

---

### Naming Rules for Channels / Regions

- Names MUST be canonical and stable.
- Names MUST be treated as part of the protocol surface (drift is a break).
- Naming convention MUST be declared here (example: `yai.<layer>.<purpose>`).

---

## Enforcement / Mechanism

L0 must be enforceable via non-negotiable checks:

- Compile-time offset checks in vault headers (static asserts on offsets/sizes)
- Protocol header constants defined in `specs/protocol/*`
- Kernel startup validation:
  - header signature check
  - version + layout compatibility check
  - region map check (bounds + alignment)
  - optional integrity check

A component that bypasses L0 checks is non-compliant by definition.

---

## Interfaces

This boundary binds the following files (non-exhaustive):

- `specs/protocol/README.md`
- `specs/protocol/protocol.h`, `commands.h`, `transport.h`
- `kernel/include/yai_vault.h`
- `yai-engine/include/shared_constants.h`
- `yai-mind/src/shared/constants.rs`

All interface files MUST trace their layout/IDs back to this L0 document.

---

## Compatibility Policy

Changes are classified as:

**Compatible**

- adding new fields in reserved/extended regions gated by version
- adding new IDs without reuse
- adding new regions with explicit offsets and version gating

**Breaking**

- modifying existing offsets/sizes/packing behavior
- reordering fields in shared structs
- reusing an ID
- changing naming rules or canonical names

Breaking changes MUST bump version_major and update all interface surfaces.

---

## Failure Modes

- Offset drift breaks shared memory interpretation (cross-language mismatch)
- ID mismatch causes invalid command/state mapping and undefined behavior
- Naming drift fragments the system into incompatible “sub-vaults”
- Version ambiguity makes reproducibility and governance non-defensible

---

## Traceability

L0 must remain compliant with:

- `invariants/I-001-traceability.md`
- `invariants/I-002-determinism.md`
- `invariants/I-003-governance.md`

In particular: changes to L0 MUST be attributable, versioned, and reconstructable.
