# Runbook D — Hostile/Boundary Baseline

## Goal

Demonstrate refusal/degraded behavior and boundary baseline checks.

## Steps

```bash
cd /Users/francescomaiomascio/Developer/YAI/cli
./dist/bin/yai up

# no active workspace -> refusal
./dist/bin/yai ws unset || true
./dist/bin/yai ws run payment.authorize provider=bank

# boundary scripts from yai repo
cd /Users/francescomaiomascio/Developer/YAI/yai
./tests/integration/workspace/workspace_hostile_path_baseline.sh
./tests/integration/workspace/workspace_isolation_guards.sh
./tests/integration/workspace/workspace_negative_paths.sh
```

## Acceptance

- runtime refuses ambiguous or unbound execution
- hostile baseline scripts pass
- degraded/unsupported mode fields remain explicit in surfaces
