# Runtime/Enforcement Alignment Audit

## Scope
This audit links formal semantics to runtime enforcement application.

## Runtime Enforcement Inputs
- Envelope validity (`yai_validate_envelope_v1`)
- Authority command gate + policy gate
- Governance final effect and review state
- Runtime capability/workspace binding

## Required Formal Counterparts
- authority admissibility invariants
- resolution precedence invariants
- policy application invariants
- grants validity invariants
- containment mode invariants
- review/escalation transition invariants

## Alignment Outcome
The formal layer now includes dedicated modules for all above surfaces and a
linkage matrix in `formal/traceability/enforcement-linkage.json`.
