# RUNBOOK

Launch ID: `SC102-WAVE1-LAUNCH`
Frozen golden wave: `WAVE-1-2026-02-25-0e7af41`

## Demo objective

Demonstrate, in 10-15 minutes, that YAI can run one governed cross-domain wave (D1 + D8), produce auditable evidence, and verify it deterministically.

## Pre-demo checks

1. Runtime and CLI

```bash
yai --version
yai down --ws dev --force || true
yai up --ws dev --detach --allow-degraded
yai status --json
yai doctor --json
```

2. Repository and pins

```bash
cd ~/Developer/YAI/yai
git rev-parse --short=12 HEAD
tools/bin/yai-check-pins
```

## Live script (record this)

1. Run SC102 Wave 1

```bash
cd ~/Developer/YAI/yai
./docs/40-qualification/QT-0.1-003-SC102-WAVE1/run/run-wave.sh
```

2. Open latest bundle and verify

```bash
LATEST="$(cat docs/40-qualification/WAVES/LATEST)"
cd "docs/40-qualification/WAVES/$LATEST"
./verify/verify.sh
sed -n 1,120p INDEX.md
```

3. Show runtime logs (optional, if asked)

```bash
WROOT="$(ls -1dt ~/.yai/qualifications/SC102/WAVE-* | head -n 1)"
STAMP="$(ls -1dt "$WROOT"/* | head -n 1)"
ls -1 "$STAMP/logs"
tail -n 40 "$STAMP/logs/"*.log
```

## What to emphasize

- One runner executes both domains with identical governance grammar.
- Bundle includes release identity and verification script.
- Commercial claims map to concrete `wave_id` evidence.

## What to avoid

- Avoid discussing future domains as already delivered.
- Avoid claiming engine-per-workspace topology.
- Avoid claims without showing `verify.sh` pass and `INDEX.md` rows.

## Expected outcomes

- Wave runner exits `0`.
- Bundle verify exits `0`.
- `INDEX.md` includes selected D1 and D8 runs.
