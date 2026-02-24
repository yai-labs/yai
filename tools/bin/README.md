# tools/bin (Hard-Cut Wrappers)

`yai` keeps stable command entrypoints here, but canonical implementation is externalized.

- Primary target: `yai-infra/tools/bin/*`
- Extended targets: `yai-infra/tools/release/*`, `yai-infra/tools/bundle/*`

Behavior:
- Wrappers are hard-delegated.
- If canonical target is missing, wrapper exits with `2` and prints missing path.
