# Foundation Mapping Matrix

Action vocabulary:
- `keep`
- `move`
- `rename`
- `merge`
- `split`
- `remove`
- `keep-temporary`

| Legacy path | Target path | Action | Rationale | Dependency impact | Follow-up needed |
|---|---|---|---|---|---|
| `kernel/src/core/ids.c` | `lib/support/ids.c` | `move` | runtime id generation is shared primitive | kernel now consumes support module | verify all includes use `include/yai/support/ids.h` |
| `kernel/src/core/logger.c` | `lib/support/logger.c` | `move` | logging helper is cross-cutting infra utility | core/exec/brain can reuse same logger module | split structured logger policy later if needed |
| `runtime-protocol/src/rpc_runtime.c` | `lib/protocol/rpc_runtime.c` | `move` | runtime RPC envelope helpers are protocol foundation | runtime-protocol Makefile now compiles shared protocol source | retire dedicated runtime-protocol source tree later |
| `kernel/src/core/rpc_codec.c` | `lib/protocol/rpc_codec.c` | `move` | protocol error-frame encoding belongs to protocol utility layer | kernel now links protocol codec module | remove core-dispatch coupling by introducing protocol writer abstraction |
| `kernel/src/core/rpc_binary.c` | `lib/protocol/rpc_binary.c` + `kernel/src/core/rpc_binary.c` | `split` | legacy file mixes protocol framing and core business logic | new protocol primitive extracted; kernel handler remains temporary | move business handler into `lib/core/dispatch` in core wave |
| `kernel/include/transport.h` | `include/yai/protocol/transport_contract.h` + platform/core internals | `split` | contract vs runtime operation concerns differ | clearer protocol contract ownership | finish transport serving boundary split |
| `mind/src/transport/protocol.c` | (temporary) `mind/src/transport/protocol.c` | `keep-temporary` | currently brain-specific transport flow | no forced cross-domain dependency introduced | split if/when reused by non-brain modules |
| `mind/src/memory/arena.c` | (temporary) `mind/src/memory/arena.c` | `keep-temporary` | arena currently tied to brain memory lifecycle | avoids premature support-layer leakage | evaluate `split` into generic support arena + brain adapter |
| `mind/include/mind_error.h` | `include/yai/support/errors.h` bridge + mind taxonomy | `split` | shared callers need stable include path, taxonomy still brain-owned | support bridge available without forcing taxonomy rewrite | define unified repository error taxonomy later |
| `kernel/src/core/*` ids/logger/rpc files in Makefile | `lib/support` and `lib/protocol` source paths | `move` | build should consume extracted foundations | dependency direction cleaner for next refactors | remove legacy include roots progressively |
