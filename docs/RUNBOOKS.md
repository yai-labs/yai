# Runbooks Index

Canonical execution sequence for deterministic verification.

## Step 1: Foundation (L0..L2)

```bash
./scripts/verify/law-kernel.sh
./scripts/verify/core.sh
```

## Step 2: Runtime Gates (L3)

```bash
./scripts/gates/ws.sh dev
./scripts/gates/cortex.sh dev
./scripts/gates/events.sh
./scripts/gates/graph.sh dev
```

Providers gate modes:
```bash
# default non-strict
./scripts/gates/providers.sh dev

# strict
REQUIRE_ACTIVE_PROVIDER=1 ./scripts/gates/providers.sh dev
```

## Step 3: Product Surface (L4)

```bash
yai verify core
yai verify full
yai test smoke --ws dev --timeout-ms 8000
```

## Step 4: Full Level Suite (L5)

```bash
DATASET_GATE=1 ./scripts/suites/levels/l0-l7.sh
```

## Step 5: Ops Suite (No LLM)

```bash
./scripts/suites/ops/no-llm-360.sh
```

## Existing Detailed Runbooks

- `docs/runbooks/TEST_EVENTS.md`
- `docs/runbooks/TEST_GRAPH.md`
- `docs/runbooks/TEST_PROVIDERS.md`
- `docs/runbooks/TEST_HARDFAIL.md`
- `docs/runbooks/TEST_EMBED.md`
