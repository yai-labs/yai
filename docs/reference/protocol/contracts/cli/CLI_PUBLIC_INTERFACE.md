# YAI CLI Public Interface (v1)

This surface is normative for public command compatibility.

## Compatibility principle

Public CLI shape can remain stable while internal runtime topology converges to:
- `core` (sovereign control)
- `exec` (execution plane)
- `brain` (cognitive plane)

## Boundary rule

CLI is a client surface.
It must not redefine authority semantics or bypass canonical protocol control schemas and protocol contract headers.

## Historical aliases

Legacy command group names may remain temporarily for compatibility.
They are interpreted as aliases, not ontology primitives.

## Governance-first workspace controls

The following surfaces are part of the canonical governance control path:

- `yai govern list [kind]`
- `yai govern inspect <object-id>`
- `yai govern validate [registry|objects|all]`
- `yai govern gov source import --path <file-or-dir> [--path ...]`
- `yai govern gov source list`
- `yai govern gov source inspect <source-id>`
- `yai govern gov parse <source-id>`
- `yai govern gov parsed list|inspect <parsed-id>`
- `yai govern gov normalize <parsed-id>`
- `yai govern gov normalized list|inspect <normalized-id>`
- `yai govern gov build <normalized-id>`
- `yai govern gov candidate list|inspect <candidate-id>`
- `yai govern gov validate <candidate-id>`
- `yai govern gov diff <candidate-id> --against <candidate-id>`
- `yai govern gov status <source-id|candidate-id>`
- `yai govern gov review status|inspect|submit|approve|reject|withdraw <candidate-id>`
- `yai ws policy dry-run <object-id>`
- `yai ws policy attach <object-id>`
- `yai ws policy activate <object-id>`
- `yai ws policy detach <object-id>`
- `yai ws policy effective`

These surfaces are object-centric and must resolve IDs via governance registry indexes,
not direct repository path UX.
