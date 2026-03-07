# Foundation Layers Extraction

## Purpose
This wave extracts cross-cutting foundations from legacy topology into:
- `lib/support`
- `lib/platform`
- `lib/protocol`

and aligns their public contracts under:
- `include/yai/support`
- `include/yai/platform`
- `include/yai/protocol`

## Why these are foundation layers
### support
Reusable primitives with no domain policy ownership (ids, logging utility, strings/paths helpers, generic errors).

### platform
Host interaction wrappers only (OS pid, filesystem checks/creation, UDS listening, clock).

### protocol
Shared wire/runtime protocol boundaries (RPC envelope helpers, codec helpers, binary frame validation, protocol IDs).

## What was extracted in this wave
### Extracted to `lib/support`
- `kernel/src/core/ids.c` -> `lib/support/ids.c`
- `kernel/src/core/logger.c` -> `lib/support/logger.c`
- new support modules:
  - `lib/support/errors.c`
  - `lib/support/strings.c`
  - `lib/support/paths.c`
  - `lib/support/internal.h`

### Extracted to `lib/protocol`
- `runtime-protocol/src/rpc_runtime.c` -> `lib/protocol/rpc_runtime.c`
- `kernel/src/core/rpc_codec.c` -> `lib/protocol/rpc_codec.c`
- new protocol modules:
  - `lib/protocol/rpc_binary.c` (framing validation primitive)
  - `lib/protocol/message_types.c`
  - `lib/protocol/internal.h`

### Added to `lib/platform`
- `lib/platform/os.c`
- `lib/platform/fs.c`
- `lib/platform/uds.c`
- `lib/platform/clock.c`
- `lib/platform/internal.h`

## Public header alignment
Consolidated/updated headers:
- `include/yai/support/{ids.h,logger.h,errors.h,paths.h,strings.h,arena.h}`
- `include/yai/platform/{os.h,fs.h,uds.h,clock.h}`
- `include/yai/protocol/{rpc_runtime.h,rpc_codec.h,rpc_binary.h,message_types.h,transport_contract.h}`

## Ambiguous cases and decisions
### `mind/src/memory/arena.c`
Decision: **keep brain-specific (temporary)**.

Rationale:
- current arena implementation is tightly coupled to mind memory lifecycle and tests,
- extraction now would blur domain ownership while `brain` refoundation is still pending.

Follow-up:
- evaluate split in next wave (`support/arena` generic core + brain-specific wrappers).

### `mind/src/transport/protocol.c`
Decision: **keep brain-specific (temporary)**.

Rationale:
- currently tied to brain transport flow and cognition request handling,
- not yet a runtime-wide protocol contract.

Follow-up:
- split shared protocol primitives into `lib/protocol` only when reused outside brain.

### `kernel/include/transport.h`
Decision: **split classification**.

Rationale:
- transport frame contract maps to `include/yai/protocol/transport_contract.h`,
- runtime serving/listen concerns map to platform/core boundaries depending on ownership.

### `mind/include/mind_error.h`
Decision: **keep brain-owned taxonomy with support bridge**.

Rationale:
- error codes are still brain-oriented,
- `include/yai/support/errors.h` provides temporary shared bridge with explicit follow-up.

### `kernel/src/core/rpc_binary.c`
Decision: **keep-temporary in kernel/core** with extracted protocol primitive in `lib/protocol/rpc_binary.c`.

Rationale:
- file contains workspace/control/business semantics, not pure protocol foundation,
- forcing full move now would violate layering constraints.

Follow-up:
- split pure protocol framing helpers out, keep control/business handling under `core`.

## Build and dependency impact
- root Makefile now builds foundation archives:
  - `build/lib/libyai_support.a`
  - `build/lib/libyai_platform.a`
  - `build/lib/libyai_protocol.a`
- `yai-core` links against those archives.
- kernel build now consumes extracted foundation sources from `lib/support` and `lib/protocol`.
- runtime-protocol build now compiles `../lib/protocol/rpc_runtime.c`.

## What remains temporary
- legacy top-level domain folders still exist,
- several headers still use transition wrappers,
- complete dependency cleanup across `core/exec/brain` deferred to next waves.
