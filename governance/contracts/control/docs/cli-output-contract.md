# YAI CLI Output Contract v1

Canonical envelope source:
- `contracts/control/schema/exec_reply.v1.json`

## Primary semantic interpretation

- Control authority source: `core`
- Execution reply source: `exec` (under `core` governance)
- Cognitive reply source (if applicable): `brain` attached through governed control path

## Compatibility posture

The schema accepts legacy status fields and legacy plane aliases.
CLI/SDK must normalize output deterministically to the canonical shape.

## Canonical output fields

Required canonical fields:
- `outcome`
- `code`
- `summary`
- `hint`/`hints`
- `data`
- `trace`
- `meta`
