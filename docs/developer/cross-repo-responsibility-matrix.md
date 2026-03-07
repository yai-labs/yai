# Cross-Repo Responsibility Matrix

| repo | primary role | allowed dependencies | forbidden dependencies | authority relationship | compatibility relationship | artifact/export relationship | notes |
|---|---|---|---|---|---|---|---|
| `yai-law` | normative source of truth | none | all structural deps | authoritative for law semantics | publishes compatibility surfaces consumed by others | may publish exported law snapshots/manifests | autonomous repo |
| `yai` | integration/runtime authority | `yai-law` tight link allowed | reciprocal/multi-satellite pins | consumes law as integration baseline | validates integrated behavior against law | may vendor/export integration artifacts | only repo allowed tight law link |
| `yai-sdk` | public programmatic surface consumer | no structural cross-repo pin | pinning `yai-law`, pinning `yai-cli` | non-authoritative for law | declares supported compatibility | may consume exported/generated law artifacts (optional) | must not be registry/law-live coupled |
| `yai-cli` | operator surface consumer | no structural cross-repo pin | pinning `yai-law`, pinning `yai-sdk` | non-authoritative for law/sdk | declares supported compatibility | may run verify-only compatibility checks | verify hooks are not repo dependency |
