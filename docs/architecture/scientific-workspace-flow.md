# Scientific Workspace Flow

This runbook is the canonical scientific workspace flow after WS-12 hardening.

## Preconditions

- runtime up: `./build/bin/yai` in one shell or `./dist/bin/yai up` from CLI
- active workspace binding

## Flow

1. Create and bind workspace

```bash
yai ws create ws_science_demo
yai ws set ws_science_demo
```

2. Declare scientific context

```bash
yai ws domain set --family scientific --specialization parameter-governance
```

3. Run parameter mutation without lock (expected deny)

```bash
yai ws run experiment.parameters.update dataset=chem-v3 result=run-17
```

4. Run publication with partial context (expected quarantine/deny)

```bash
yai ws run experiment.result.publish dataset=chem-v3 result=run-17 contract=approved
```

5. Run publication with reproducibility context (expected review/allow depending overlays)

```bash
yai ws run experiment.result.publish dataset=chem-v3 result=run-17 contract=approved params_hash=cfg-22 proofpack=sha256:abc lineage=dataset:chem-v3/run:17
```

6. Inspect effective policy and debug

```bash
yai ws policy effective
yai ws debug resolution
yai ws inspect
```

## Operator signals to verify

- scientific family/specialization are visible in run output
- scientific summaries are visible in policy/debug/inspect
- publication control semantics move between deny/quarantine/review based on context completeness
