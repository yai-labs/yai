# Binding — CLI Surface

CLI is a compatibility surface over canonical registries and control/protocol contracts.

## Key rule

CLI stability does not imply legacy ontology stability.
Internal runtime can converge to `core/exec/brain` while command IDs and UX remain compatible.

## Canonical dependencies

- `registry/commands.v1.json`
- `registry/schema/commands.v1.schema.json`
- `contracts/control/schema/exec_reply.v1.json`
- `contracts/protocol/include/*`
- workspace command topology: `docs/architecture/workspace-command-topology.md`
- workspace command IDs: `yai.workspace.*` (runtime control-call surface)
