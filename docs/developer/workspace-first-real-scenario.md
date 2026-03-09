# Workspace First Real Scenario (economic/payments)

This runbook executes the first real workspace-driven normative flow.

## Prerequisites

- build binary: `make yai`
- runtime socket available (`$YAI_RUNTIME_INGRESS` or `~/.yai/run/control.sock`)

## Steps

1. Create and set workspace:

```bash
yai ws create ws_real_demo
yai ws set ws_real_demo
```

2. Declare workspace domain context:

```bash
yai ws domain set --family economic --specialization payments
```

3. Inspect baseline state:

```bash
yai ws current
yai ws status
yai ws inspect
```

4. Execute one real action through workspace context:

```bash
yai ws run payment.authorize provider=bank resource=money-transfer amount=1250 authority=supervisor
```

5. Inspect post-action normative outputs:

```bash
yai ws policy effective
yai ws debug resolution
yai ws inspect
```

## What to verify

- `family_effective = economic`
- `specialization_effective = payments`
- effect is emitted (`allow | review_required | quarantine | deny`)
- inspect includes updated `last_resolution_summary`
- debug surface includes declared/inferred/effective summaries

## Cleanup

```bash
yai ws clear              # clear runtime-local state, keep binding
yai ws unset              # unbind active workspace from session
yai ws destroy ws_real_demo
```
