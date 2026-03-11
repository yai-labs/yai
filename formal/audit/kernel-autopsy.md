# Kernel Autopsy: `YAI_KERNEL.tla`

## Findings
- Mixed concerns in a single file:
  - bootstrap/runtime state machine
  - authority constraints
  - compliance flag gates
  - energy accounting
  - external effect checks
- Imported `GOVERNANCE_IDS` for vault offsets but with no module decomposition.
- Contained no explicit runtime policy/grant/containment submodels.

## Why It Is No Longer Canonical
- System architecture is no longer kernel-centric.
- Runtime is split into clear domains (`authority`, `policy`, `grants`, `containment`, `workspace`, `dispatch`).
- Governance resolution and protocol control semantics require independent modules.

## Action Taken
- Kernel monolith moved to `formal/legacy/tla/YAI_KERNEL.tla`.
- Replaced by decomposed module set under `formal/modules/`.
- New root model: `formal/models/yai_system.tla`.
