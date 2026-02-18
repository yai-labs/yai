# Tooling Layout Migration

Repository: `yai`
Goal: remove legacy `scripts` tree and make `tools` + `tests` canonical.

## Mapping Table

| OLD_PATH | NEW_PATH | CLASS | NOTES |
|---|---|---|---|
| scripts:yai-doctor | tools/bin/yai-doctor | entrypoint | user-facing command |
| scripts:yai-purge | tools/bin/yai-purge | entrypoint | user-facing command |
| scripts:yai-verify | tools/bin/yai-verify | entrypoint | user-facing command |
| scripts:yai-gate | tools/bin/yai-gate | entrypoint | user-facing command |
| scripts:yai-suite | tools/bin/yai-suite | entrypoint | user-facing command |
| scripts:ops/gate.sh | tools/ops/gate/gate.sh | ops | operator wrapper |
| scripts:ops/suite.sh | tools/ops/suite/suite.sh | ops | operator wrapper |
| scripts:ops/verify.sh | tools/ops/verify/verify.sh | ops | operator wrapper |
| scripts:gates/cortex.sh | tools/ops/gate/cortex.sh | ops | gate implementation |
| scripts:gates/dataset-global-stress.sh | tools/ops/gate/dataset-global-stress.sh | ops | gate implementation |
| scripts:gates/events.sh | tools/ops/gate/events.sh | ops | gate implementation |
| scripts:gates/graph.sh | tools/ops/gate/graph.sh | ops | gate implementation |
| scripts:gates/providers.sh | tools/ops/gate/providers.sh | ops | gate implementation |
| scripts:gates/providers-modes-test.sh | tools/ops/gate/providers-modes-test.sh | test | deterministic pass/fail gate test |
| scripts:gates/ws.sh | tools/ops/gate/ws.sh | ops | gate implementation |
| scripts:gate-cortex.sh | tools/ops/gate-cortex.sh | ops | wrapper retained by name |
| scripts:gate-dataset-global-stress.sh | tools/ops/gate-dataset-global-stress.sh | ops | wrapper retained by name |
| scripts:gate-graph.sh | tools/ops/gate-graph.sh | ops | wrapper retained by name |
| scripts:gate-providers.sh | tools/ops/gate-providers.sh | ops | wrapper retained by name |
| scripts:gate-ws.sh | tools/ops/gate-ws.sh | ops | wrapper retained by name |
| scripts:suites/levels/l0-l7.sh | tools/ops/suite/levels/l0-l7.sh | ops | suite implementation |
| scripts:suites/ops/no-llm-360.sh | tools/ops/suite/ops/no-llm-360.sh | ops | suite implementation |
| scripts:suites/ops/fault-injection-v1.sh | tools/ops/suite/ops/fault-injection-v1.sh | ops | suite implementation |
| scripts:suites/ops/perf-slo-v1.sh | tools/ops/suite/ops/perf-slo-v1.sh | ops | suite implementation |
| scripts:suites/ops/recovery-compat-v1.sh | tools/ops/suite/ops/recovery-compat-v1.sh | ops | suite implementation |
| scripts:suites/ops/security-sanity-v1.sh | tools/ops/suite/ops/security-sanity-v1.sh | ops | suite implementation |
| scripts:suites/ops/stress-v1.sh | tools/ops/suite/ops/stress-v1.sh | ops | suite implementation |
| scripts:suite-l0-l7.sh | tools/ops/suite-l0-l7.sh | ops | top-level wrapper retained by name |
| scripts:suite-ops-360-no-llm.sh | tools/ops/suite-ops-360-no-llm.sh | ops | top-level wrapper retained by name |
| scripts:verify/core.sh | tools/ops/verify/core.sh | ops | verify implementation |
| scripts:verify/law-kernel.sh | tools/ops/verify/law-kernel.sh | ops | verify implementation |
| scripts:verify-core.sh | tools/ops/verify-core.sh | ops | wrapper retained by name |
| scripts:verify-events.sh | tools/ops/verify-events.sh | ops | wrapper retained by name |
| scripts:verify-law-kernel.sh | tools/ops/verify-law-kernel.sh | ops | wrapper retained by name |
| scripts:check-generated.sh | tools/dev/check-generated.sh | dev | duplicate wrapper removed |
| scripts:dev/check-generated.sh | tools/dev/check-generated.sh | dev | canonical copy |
| scripts:dev/resolve-yai-bin.sh | tools/dev/resolve-yai-bin.sh | dev | shared helper |
| scripts:protocol_tester | tools/dev/protocol_tester | dev | duplicate wrapper removed |
| scripts:dev/protocol_tester | tools/dev/protocol_tester | dev | canonical copy |
| scripts:gen-vault-abi | tools/dev/gen-vault-abi | dev | duplicate wrapper removed |
| scripts:dev/gen-vault-abi | tools/dev/gen-vault-abi | dev | canonical copy |
| scripts:dev/yai-doctor | tools/dev/yai-doctor | dev | internal impl used by entrypoint |
| scripts:dev/yai-purge | tools/dev/yai-purge | dev | internal impl used by entrypoint |
| scripts:release/bump_version.sh | tools/release/bump_version.sh | release | release tool |
| scripts:release/check_pins.sh | tools/release/check_pins.sh | release | release tool |
| scripts:release/pin_cli.sh | tools/release/pin_cli.sh | release | release tool |
| scripts:data/fetch-embeddings.sh | tools/data/fetch-embeddings.sh | data | canonical copy |
| scripts:fetch-embeddings.sh | (deleted) | data | duplicate wrapper removed |
| scripts:data/dataset-global-stress.sh | tools/data/dataset-global-stress.sh | data | data gate wrapper |
| data:datasets/global-stress/v1/scripts/import-seed-via-cli.sh | tools/data/global-stress/v1/import-seed-via-cli.sh | data | tooling moved out of data |
| data:datasets/global-stress/v1/scripts/load-events-log.sh | tools/data/global-stress/v1/load-events-log.sh | data | tooling moved out of data |
| data:datasets/global-stress/v1/scripts/README.md | tools/data/global-stress/v1/README.md | data | updated tooling location |
| scripts:test_handshake.py | tests/integration/test_handshake.py | test | duplicate consolidated |
| scripts:dev/test_handshake.py | (deleted) | test | duplicate removed |
| scripts:bundle/build_bundle.sh | tools/bundle/build_bundle.sh | dev | bundling helper |
| scripts:bundle/manifest.sh | tools/bundle/manifest.sh | dev | bundling helper |
| scripts:bundle/README.md | tools/bundle/README.md | dev | bundling helper docs |
| scripts:README.md | tools/README.md | dev | canonical layout docs |
| scripts:lib/logging.sh | tools/lib/logging.sh | dev | shared helper |
| scripts:fault-injection-v1.sh | tools/ops/fault-injection-v1.sh | ops | wrapper retained by name |
| scripts:perf-slo-v1.sh | tools/ops/perf-slo-v1.sh | ops | wrapper retained by name |
| scripts:recovery-compat-v1.sh | tools/ops/recovery-compat-v1.sh | ops | wrapper retained by name |
| scripts:security-sanity-v1.sh | tools/ops/security-sanity-v1.sh | ops | wrapper retained by name |
| scripts:stress-v1.sh | tools/ops/stress-v1.sh | ops | wrapper retained by name |
