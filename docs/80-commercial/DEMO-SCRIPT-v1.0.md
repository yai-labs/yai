# SC102 Demo Script v1.0

Launch ID: `SC102-WAVE1-LAUNCH`
Golden Wave: `WAVE-1-2026-02-25-0e7af41`

## Duration

10-15 minutes.

## Step 1 - Runtime health

```bash
yai down --ws dev --force || true
yai up --ws dev --detach --allow-degraded
yai status --json
yai doctor --json
```

Expected: `overall=READY`.

## Step 2 - Execute star case wave

```bash
cd ~/Developer/YAI/yai
./docs/40-qualification/QT-0.1-003-SC102-WAVE1/run/run-wave.sh
```

Expected: `PASS: wave run complete`.

## Step 3 - Open frozen bundle

```bash
cd ~/Developer/YAI/yai/docs/40-qualification/WAVES/WAVE-1-2026-02-25-0e7af41
./verify/verify.sh
```

Expected: `PASS: Wave bundle verified`.

## Step 4 - Show KPI proof table

```bash
sed -n '1,120p' INDEX.md
```

Expected highlights:

- D1: deny, `connect_established=false`, `bytes_exfiltrated=0`
- D8: deny, `outputs_persisted=false`, `bytes_written=0`, `artifacts_delta=0`

## Step 5 - Show release identity

```bash
python3 - <<'PY'
import json
j=json.load(open('MANIFEST.json'))
print(j['product_release'])
PY
```

Expected: yai/yai-cli/specs identity present.

## Step 6 - Closing statement

State that all customer claims map to launch ID `SC102-WAVE1-LAUNCH` and to the verified wave bundle above.
