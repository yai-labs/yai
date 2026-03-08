# Cross-Repo Pinning / Compatibility Matrix

| source repo | target repo | relation type | allowed? | rationale | implementation mode |
|---|---|---|---|---|---|
| `yai` | `law` | `pin` | yes | integration runtime needs reproducible law baseline | submodule/vendor lock/baseline lock |
| `yai` | `law` | `baseline-lock` | yes | stable integration cut | pinned baseline metadata |
| `yai` | `law` | `compatibility-declaration` | yes | explicit supported range | docs + compatibility files |
| `sdk` | `law` | `pin` | no | satellite repos must not structurally depend on law | forbidden |
| `sdk` | `law` | `compatibility-declaration` | yes | sdk declares supported law surfaces | docs/manifests |
| `sdk` | `law` | `generated-artifact-consumption` | yes | optional exported baseline consumption | tooling/verify/export snapshots |
| `sdk` | `law` | `verify-only` | yes | compatibility checks only | optional tooling checks |
| `cli` | `law` | `pin` | no | cli must not structurally bind to law repo | forbidden |
| `cli` | `law` | `compatibility-declaration` | yes | operator surface compatibility statement | docs/manifests |
| `cli` | `law` | `verify-only` | yes | compatibility checks without dependency | verify tooling |
| `cli` | `sdk` | `pin` | no | avoid satellite-to-satellite structural coupling | forbidden |
| `cli` | `sdk` | `compatibility-declaration` | yes | declare expected sdk public surface | docs/manifests |
| `law` | any | `pin` | no | law repo must remain autonomous | forbidden |
