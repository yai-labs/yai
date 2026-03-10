# Graph Truth and Transient Knowledge Walkthrough (DP-7)

This walkthrough validates workspace-bound runtime writes for:
- persistent graph truth, and
- transient knowledge-state heat.

Note: file name contains legacy alias for continuity. The canonical runtime model
is unified (`core`, `exec`, `data`, `graph`, `knowledge`) and workspace-first.

## Goal

Show one governed workspace action producing:
- persisted graph state (`graph` family),
- transient activation/working-set state (`knowledge` support),
- inspectable refs through runtime surfaces.

## Scenario

1. Create/select a workspace and set `digital/remote-publication` context.
2. Run governed action (`digital.publish ...`).
3. Verify runtime produced event/decision/evidence records.
4. Verify graph sink records under runtime graph path.
5. Verify transient knowledge-state records under runtime transient path.
6. Verify inspect/effective/debug surfaces expose graph/transient refs.

## Persistent graph artifacts

- `~/.yai/run/<ws>/runtime/graph/persistent-nodes.v1.ndjson`
- `~/.yai/run/<ws>/runtime/graph/persistent-edges.v1.ndjson`
- `~/.yai/run/<ws>/runtime/graph/index.v1.json`

## Transient knowledge artifacts

- `~/.yai/run/<ws>/runtime/transient/activation-state.v1.ndjson`
- `~/.yai/run/<ws>/runtime/transient/working-set.v1.ndjson`
- `~/.yai/run/<ws>/runtime/transient/index.v1.json`

## What to verify

- Graph refs (`bgn-*`, `bge-*`) are emitted and persisted.
- Transient refs (`btc-*`, `bws-*`) are emitted and persisted.
- Graph index is authoritative (`graph_truth_authoritative=true`).
- Transient state is non-authoritative (`transient_authoritative=false`).
- Runtime surfaces expose both stores and last refs for active workspace.

## Degraded behavior expectation

If transient backend degrades, graph/evidence/governance correctness remains
valid; only transient knowledge-state quality may degrade.
