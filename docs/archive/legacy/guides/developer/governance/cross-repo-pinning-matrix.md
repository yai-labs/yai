# Cross-Repo Pinning / Compatibility Matrix

| source repo | target repo | relation type | allowed? | rationale | implementation mode |
|---|---|---|---|---|---|
| `yai` | `governance` | `pin` | yes | integration runtime needs reproducible governance baseline | submodule/vendor lock/baseline lock |
| `yai` | `governance` | `baseline-lock` | yes | stable integration cut | pinned baseline metadata |
| `yai` | `governance` | `compatibility-declaration` | yes | explicit supported range | docs + compatibility files |
| `sdk` | `governance` | `pin` | no | satellite repos must not structurally depend on governance | forbidden |
| `sdk` | `governance` | `compatibility-declaration` | yes | sdk declares supported governance surfaces | docs/manifests |
| `sdk` | `governance` | `generated-artifact-consumption` | yes | optional exported baseline consumption | tooling/verify/export snapshots |
| `sdk` | `governance` | `verify-only` | yes | compatibility checks only | optional tooling checks |
| `cli` | `governance` | `pin` | no | cli must not structurally bind to governance repo | forbidden |
| `cli` | `governance` | `compatibility-declaration` | yes | operator surface compatibility statement | docs/manifests |
| `cli` | `governance` | `verify-only` | yes | compatibility checks without dependency | verify tooling |
| `cli` | `sdk` | `pin` | no | avoid satellite-to-satellite structural coupling | forbidden |
| `cli` | `sdk` | `compatibility-declaration` | yes | declare expected sdk public surface | docs/manifests |
| `governance` | any | `pin` | no | governance repo must remain autonomous | forbidden |
