# Runtime Enforcement

`lib/runtime/enforcement/` is the runtime-side policy application and control-call
enforcement surface.

Formal bridge:

- invariant classes are mapped in `formal/traceability/enforcement-linkage.json`
- matrix narrative lives in `formal/audit/enforcement-invariant-matrix.md`
- outcome taxonomy exposed in `include/yai/runtime/enforcement.h`

Operational role:

- validate control envelope contract assumptions
- apply authority + governance-resolved effect gates
- emit deterministic runtime decision classes (`allow`, `review_required`, `blocked`, `deny`, `quarantined`)
