# Cross-Repo Pinning / Compatibility Matrix

| source repo | target repo | relation type | allowed? | rationale | implementation mode |
|---|---|---|---|---|---|
| `yai` | `yai-law` | `pin` | yes | integration runtime needs reproducible law baseline | submodule/vendor lock/baseline lock |
| `yai` | `yai-law` | `baseline-lock` | yes | stable integration cut | pinned baseline metadata |
| `yai` | `yai-law` | `compatibility-declaration` | yes | explicit supported range | docs + compatibility files |
| `yai-sdk` | `yai-law` | `pin` | no | satellite repos must not structurally depend on law | forbidden |
| `yai-sdk` | `yai-law` | `compatibility-declaration` | yes | sdk declares supported law surfaces | docs/manifests |
| `yai-sdk` | `yai-law` | `generated-artifact-consumption` | yes | optional exported baseline consumption | tooling/verify/export snapshots |
| `yai-sdk` | `yai-law` | `verify-only` | yes | compatibility checks only | optional tooling checks |
| `yai-cli` | `yai-law` | `pin` | no | cli must not structurally bind to law repo | forbidden |
| `yai-cli` | `yai-law` | `compatibility-declaration` | yes | operator surface compatibility statement | docs/manifests |
| `yai-cli` | `yai-law` | `verify-only` | yes | compatibility checks without dependency | verify tooling |
| `yai-cli` | `yai-sdk` | `pin` | no | avoid satellite-to-satellite structural coupling | forbidden |
| `yai-cli` | `yai-sdk` | `compatibility-declaration` | yes | declare expected sdk public surface | docs/manifests |
| `yai-law` | any | `pin` | no | law repo must remain autonomous | forbidden |
