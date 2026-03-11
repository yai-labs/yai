# Control Contracts

`contracts/control/` defines canonical control-plane schemas.

Primary ontology:
- `core` is the authority/control plane.
- `exec` is the execution/external-effect plane.
- `brain` is the cognitive plane attached under governance.

Historical labels (`ingress`, `root`, `kernel`, `engine`, `mind`) remain accepted as compatibility aliases where explicitly declared.

## Canonical schemas

- `schema/control_plane.v1.json`
- `schema/control_call.v1.json`
- `schema/exec_reply.v1.json`
- `schema/authority.v1.json`
- `schema/authority.json` (deprecated alias)

## Normative requirements

- Control semantics are `core` semantics.
- `exec_reply` is the canonical response envelope for `core ↔ exec` interactions and compatible legacy flows.
- Alias terms must not be interpreted as primary ontology.
