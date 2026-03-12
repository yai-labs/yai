# User Guide Pointers

This folder contains navigation pointers.
Normative source is canonical `governance`, consumed in runtime through `governance/runtime-package`.

## Canonical pointers

- Canonical governance manifests and layers: `governance/manifests/*`
- Runtime-facing contract in repo: `governance/runtime-package/*`
- Runtime compatibility checks: `tools/bin/yai-governance-compat-check`

## Boundary rule

If behavior changes contract/spec semantics, update canonical `governance` first and regenerate embedded artifacts.
