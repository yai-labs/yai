# SC102 Demo Script v1.0

Launch ID: `SC102-WAVE1-LAUNCH`  
Canonical wave build: `WAVE-1-2026-02-25-ddb4e9e`

## Duration

10-15 minutes (wow point within first 120 seconds).

## 0. Runtime health

```bash
yai down --ws dev --force || true
yai up --ws dev --detach --allow-degraded
yai status --json
yai doctor --json
```

Expected: `overall=READY`.

## 1. Run launch wave

```bash
cd ~/Developer/YAI/yai
./docs/40-qualification/QT-0.1-003-SC102-WAVE1/run/run-wave.sh
```

Expected: `PASS: wave run complete`.

## 2. Open launch bundle and verify

```bash
cd ~/Developer/YAI/yai/docs/40-qualification/WAVES/SC102-WAVE1-LAUNCH
./verify/verify.sh
```

Expected: `PASS: Wave bundle verified`.

## 3. Show policy fingerprint (deny + allow pinned)

```bash
python3 - <<'PY'
import json
m=json.load(open('MANIFEST.json'))
print('launch_id:', m['launch_id'])
print('wave_id:', m['wave_id'])
print('policy_hash:', m['policy_hash'])
for p in m['policy_files']:
    print(f"- {p['pack_id']} {p['baseline_id']} {p['policy_hash']}")
PY
```

Expected: policy hash and per-baseline hashes printed from `POLICY/`.

## 4. Show deny vs allow business proof

```bash
sed -n '1,220p' INDEX.md
```

Expected highlights:

- D1 deny rows: `connect_established=false`, `bytes_exfiltrated=0`
- D8 deny row: `outputs_persisted=false`, `bytes_written=0`, `artifacts_delta=0`
- D8 allow row: `outputs_persisted=true`, `bytes_written>0`, `artifacts_delta>0`

## 5. Show release identity

```bash
python3 - <<'PY'
import json
m=json.load(open('MANIFEST.json'))
print(json.dumps(m['product_release'], indent=2))
PY
```

Expected: yai/yai-cli/spec pin identity present.

## 6. Colossus positioning statement

Wave-1 launch colossus is GitHub Issue Comment Agent (deny/allow on external effect).  
This release keeps D1+D8 as deterministic proof substrate and extends to GitHub effect proof in pilot execution.
