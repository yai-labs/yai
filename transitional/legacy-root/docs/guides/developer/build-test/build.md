# Build

## Canonical Commands
- `make -j4`
- `make test`
- `make mind-check`
- `make help`

## Runtime Build Surface
- CLI entrypoint: `cmd/yai/main.c`
- Edge CLI: `cmd/yai-edge/main.c`
- Runtime headers: `include/yai/runtime/`
- Runtime implementation: `lib/runtime/`

## Mind Build
- `make -C mind`
- `make -C mind clean`

Use docs-only changes to run the smallest relevant verification target before push.
