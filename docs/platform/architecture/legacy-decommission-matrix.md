# Legacy Decommission Matrix

| Legacy path | Final status | Action taken | Reason | Residual risk | Follow-up |
|---|---|---|---|---|---|
| `boot/` | removed | directory removed | lifecycle logic moved to `lib/core/lifecycle` + `cmd/yai-core` | none | none |
| `root/` | removed | directory removed | dispatch/control moved to `lib/core/dispatch` | none | none |
| `kernel/` | removed | directory removed | session/workspace behavior moved to `lib/core` | none | none |
| `engine/` | removed | directory removed | exec behavior moved to `lib/exec` | none | none |
| `mind/` | removed | directory removed | brain behavior moved to `lib/brain` | low (compat daemon stays in `cmd/legacy`) | eventually fold `cmd/legacy/yai-mind/main.c` into `cmd/yai-core/main.c` |
| `runtime-protocol/` | removed | directory removed after absorb | foundation convergence complete in `lib/protocol` | none | none |
| `tests/domains/*` | deprecated-temporary | replaced by new tests grammar | legacy taxonomy no longer valid | confusion if kept too long | remove placeholders |
| legacy bins (`yai-boot`,`yai-root-server`,`yai-kernel`,`yai-engine`,`yai-mind`) | absorbed | removed from root build outputs | primary entries are `yai`,`yai-core` | low (`yai-mind` compat test binary only) | retire compat daemon when integration no longer needs it |
| old local include trees (`*/include`) | removed | moved to `include/yai/*` | new public grammar is authoritative | none | none |
