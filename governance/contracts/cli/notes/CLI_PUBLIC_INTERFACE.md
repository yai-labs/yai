# YAI CLI Public Interface (v1)

This surface is normative for public command compatibility.

## Compatibility principle

Public CLI shape can remain stable while internal runtime topology converges to:
- `core` (sovereign control)
- `exec` (execution plane)
- `brain` (cognitive plane)

## Boundary rule

CLI is a client surface.
It must not redefine authority semantics or bypass `contracts/control/` and `contracts/protocol/`.

## Historical aliases

Legacy command group names may remain temporarily for compatibility.
They are interpreted as aliases, not ontology primitives.

## Governance-first workspace controls

The following surfaces are part of the canonical governance control path:

- `yai law list [kind]`
- `yai law inspect <object-id>`
- `yai law validate [registry|objects|all]`
- `yai law gov source import --path <file-or-dir> [--path ...]`
- `yai law gov source list`
- `yai law gov source inspect <source-id>`
- `yai law gov parse <source-id>`
- `yai law gov parsed list|inspect <parsed-id>`
- `yai law gov normalize <parsed-id>`
- `yai law gov normalized list|inspect <normalized-id>`
- `yai law gov build <normalized-id>`
- `yai law gov candidate list|inspect <candidate-id>`
- `yai law gov validate <candidate-id>`
- `yai law gov diff <candidate-id> --against <candidate-id>`
- `yai law gov status <source-id|candidate-id>`
- `yai law gov review status|inspect|submit|approve|reject|withdraw <candidate-id>`
- `yai ws policy dry-run <object-id>`
- `yai ws policy attach <object-id>`
- `yai ws policy activate <object-id>`
- `yai ws policy detach <object-id>`
- `yai ws policy effective`

These surfaces are object-centric and must resolve IDs via law registry indexes,
not direct repository path UX.
