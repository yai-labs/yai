# D8-scientific/reproducibility-parameter-lock-v1 - Scientific Domain Pack (Reproducibility Parameter Lock)

This pack defines the semantic meaning of non-reproducible scientific pipeline execution.

Runtime grammar invariant:
trigger -> context -> authority/contract -> decision -> enforcement -> evidence

## Forbidden effects (semantic)
- Running an experiment/simulation/pipeline step without a declared parameter lock (params hash).
- Running with undeclared exclusions or transformations that break reproducibility.

## Runtime effect domain (where enforcement happens)
- `control` (pipeline run requests)
- optionally `storage` (publishing results without provenance)

## Intended use
- SC-102 (core-only) qualification: prove fail-closed on missing reproducibility lock + evidence completeness.
- Safe by design: vectors are simulated pipeline runs (no external side effects required).
