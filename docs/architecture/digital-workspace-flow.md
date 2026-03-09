# Digital Workspace Flow

This runbook is the canonical digital workspace flow after WS-13 hardening.

## Preconditions

- runtime up: `./build/bin/yai` in one shell or `./dist/bin/yai up` from CLI
- active workspace binding

## Flow

1. Create and bind workspace

```bash
yai ws create ws_digital_demo
yai ws set ws_digital_demo
```

2. Declare digital context

```bash
yai ws domain set --family digital --specialization remote-publication
```

3. Run publication without authority contract (expected deny)

```bash
yai ws run digital.publish sink=external_untrusted artifact=bundle-v1
```

4. Run publication with authority but untrusted sink (expected quarantine)

```bash
yai ws run digital.publish sink=external_untrusted contract=approved artifact=bundle-v1
```

5. Run publication with authority and trusted sink (expected review_required)

```bash
yai ws run digital.publish sink=internal_trusted contract=approved artifact=bundle-v1 destination=ops_portal
```

6. Switch specialization and run retrieval/distribution checks

```bash
yai ws domain set --family digital --specialization remote-retrieval
yai ws run digital.retrieve source=trusted_repo sink=internal
yai ws domain set --family digital --specialization artifact-distribution
yai ws run digital.distribute artifact=bundle-v1 sink=external_untrusted
```

7. Inspect effective policy and debug

```bash
yai ws policy effective
yai ws debug resolution
yai ws inspect
```

## Operator signals to verify

- digital family/specialization are visible in run output
- digital summaries are visible in policy/debug/inspect
- publication/distribution semantics move between deny/quarantine/review based on sink and authority context
