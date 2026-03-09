# Runbook B — Scientific Governed Flow

## Goal

Demonstrate parameter governance, reproducibility-sensitive publication, and evidence output.

## Steps

```bash
cd /Users/francescomaiomascio/Developer/YAI/cli
./dist/bin/yai up
./dist/bin/yai ws create ws_science_demo
./dist/bin/yai ws set ws_science_demo
./dist/bin/yai ws domain set --family scientific --specialization parameter-governance

# deny: missing lock
./dist/bin/yai ws run experiment.parameters.update dataset=chem-v3 result=run-17

# review/allow: publication with context
./dist/bin/yai ws run experiment.result.publish dataset=chem-v3 result=run-17 contract=approved params_hash=cfg-22 proofpack=sha256:abc lineage=dataset:chem-v3/run:17

./dist/bin/yai ws policy effective
./dist/bin/yai ws debug resolution
./dist/bin/yai ws inspect

./dist/bin/yai ws unset
./dist/bin/yai ws destroy ws_science_demo
```

## Acceptance

- scientific specialization resolved coherently
- deny/review path changes with context completeness
- inspect/policy/debug expose scientific summaries and evidence
