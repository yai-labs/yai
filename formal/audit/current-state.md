# Formal Audit: Current State

Date: 2026-03-11

## Inventory Before Refoundation
- Primary executable model was `formal/tla/YAI_KERNEL.tla` (monolith).
- `GOVERNANCE_PRECEDENCE.tla` and `GOVERNANCE_RESOLUTION.tla` were placeholders.
- `GOVERNANCE_IDS.tla` was generated from vault ABI and mixed with runtime semantics.
- Config axis was kernel-centric (`YAI_KERNEL*.cfg`).
- Traceability file referenced split-era paths (`contracts/*`, `brain`, `exec`).

## Structural Gaps
- Formal layer did not reflect runtime canonical domains (`policy`, `grants`, `containment`).
- No explicit bridge matrix between formal invariants and runtime enforcement outcomes.
- No modular ownership by architecture domain.
- No formal check entrypoints for quick/deep/enforcement-focused runs.

## Canonical Target
- Module-driven formal spine aligned to current runtime/governance/protocol/workspace.
- Explicit enforcement linkage artifacts.
- Kernel framing relegated to `formal/legacy/` only.
