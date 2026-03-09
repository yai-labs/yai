# Runbook A — Workspace Baseline Demo

## Goal

Show workspace lifecycle and core inspectability.

## Steps

```bash
cd /Users/francescomaiomascio/Developer/YAI/cli
./dist/bin/yai down --force || true
./dist/bin/yai up
./dist/bin/yai ws create ws_demo_baseline
./dist/bin/yai ws set ws_demo_baseline
./dist/bin/yai ws current
./dist/bin/yai ws status
./dist/bin/yai ws inspect
./dist/bin/yai ws clear
./dist/bin/yai ws unset
./dist/bin/yai ws destroy ws_demo_baseline
./dist/bin/yai down --force || true
```

## Acceptance

- token visible only while active binding exists
- `current/status/inspect` coherent on binding lifecycle
- clear runtime-local reset is visible
