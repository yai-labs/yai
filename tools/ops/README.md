# tools/ops

Operational orchestration for verify, gate, and suite flows.

## Structure

- `gate/`: focused runtime/provider/event/graph checks.
- `suite/`: multi-step validation pipelines.
- `verify/`: formal and baseline verification flows.
- top-level wrappers: `gate-*.sh`, `verify-*.sh`, `suite-*.sh`.

## Quick Start

- `tools/bin/yai-gate list`
- `tools/bin/yai-suite list`
- `tools/bin/yai-verify list`
