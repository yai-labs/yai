# YAI

YAI is a **layered, sovereign runtime** for governed cognition.
It separates probabilistic intelligence from deterministic control and makes authority explicit, enforceable, and auditable.

## Quickstart

```bash
make clean && make all
./dist/bin/yai-boot
./dist/bin/yai-kernel status
```

## Repository Map

- `boot/` — root entrypoint and workspace bring-up
- `core/` — control plane core services
- `kernel/` — authority and enforcement runtime (C)
- `engine/` — deterministic execution bridge and gates (C)
- `runtime/` — protocol/runtime interfaces
- `docs/` — architecture, guides, reference, operations
- `scripts/` — verification and tooling

## Contracts (Canonical)

All contracts are defined in `deps/yai-specs` and are authoritative for protocol, control, graph, vault, and compliance.
This repo consumes those headers and JSONs directly.

## Tooling

Related repos:
- `yai-cli`
- `yai-yx`
- `yai-mind`

## Documentation

- `docs/README.md`
- API Reference (Doxygen): https://francescomaiomascio.github.io/yai/

## License

See `LICENSE`.
