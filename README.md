# YAI (C)

YAI is the systems core of the YAI platform.

It brings control, execution, and cognition into one governed architecture built for explicit authority, bounded behavior, and proof under operation.

This repository contains the primary implementation of YAI: runtime foundations, execution layers, cognition subsystems, shared protocol and support surfaces, and the program artifacts that govern their evolution.

## Platform position

YAI operates inside a disciplined platform chain:

- normative authority: `law`
- operator/programmatic flow: `operator -> cli -> sdk -> yai`
- operations/evidence: `ops`

Law defines contracts and invariants.
CLI and SDK consume that truth.
YAI realizes runtime ingress and dispatch.

## Design posture

- **Authority is explicit**
- **Behavior is bounded**
- **Execution is governable**
- **Cognition is controlled**
- **Evidence is first-class**
- **Change is deliberate**

## Scope

This repository owns the governed implementation of YAI and the program artifacts required to evolve it under control.

It does not own canonical law (`law`) or shared cross-repo governance tooling (`infra`).

## Build

```bash
make yai
make dist
```

## Test/Verify

```bash
make test
make verify
```

Primary runtime entrypoints:
- `build/bin/yai`

Repository topology is authoritative under:
- `cmd/`
- `include/yai/`
- `lib/`
- `tests/`

## Documentation

- `docs/README.md`

## Dependency discipline

Canonical law is consumed as a pinned dependency through `deps/law/`.  
CLI/SDK integration is tracked via compatibility declarations, not local `deps/*.ref` pins.

Divergence from pinned law or aligned interfaces must be corrected in implementation.

## License

Apache-2.0. See `LICENSE`, `NOTICE`, and `THIRD_PARTY_NOTICES.md`.

## Law compatibility declaration

- Human-readable declaration: `LAW_COMPATIBILITY.md`
- Machine-readable declaration: `law-compatibility.v1.json`
