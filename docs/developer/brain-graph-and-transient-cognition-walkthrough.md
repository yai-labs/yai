# Brain Graph and Transient Cognition Walkthrough (DP-7)

## Goal
Show the runtime writing both:
- persistent brain graph truth,
- transient cognition heat,
for the same governed action, with clear authority separation.

## Scenario
1. Create workspace and set `digital/remote-publication` context.
2. Run governed action (`digital.publish ...`).
3. Verify runtime produced event/decision/evidence.
4. Verify brain graph sink records (`S-7`).
5. Verify transient cognition records (`S-8`).
6. Verify inspect/effective/debug expose `brain_persistence` refs.

## Persistent graph artifacts
- `~/.yai/run/<ws>/brain/graph/persistent-nodes.v1.ndjson`
- `~/.yai/run/<ws>/brain/graph/persistent-edges.v1.ndjson`
- `~/.yai/run/<ws>/brain/graph/index.v1.json`

## Transient cognition artifacts
- `~/.yai/run/<ws>/brain/transient/activation-state.v1.ndjson`
- `~/.yai/run/<ws>/brain/transient/working-set.v1.ndjson`
- `~/.yai/run/<ws>/brain/transient/index.v1.json`

## What to verify
- Graph refs (`bgn-*`, `bge-*`) are emitted and persisted.
- Transient refs (`btc-*`, `bws-*`) are emitted and persisted.
- Graph index is authoritative (`graph_truth_authoritative=true`).
- Transient index is non-authoritative (`authoritative=false`).
- Runtime surfaces expose both stores and last refs.

## Degraded behavior expectation
If transient backend degrades, graph/evidence/governance correctness must remain valid; only cognition heat quality may degrade.
