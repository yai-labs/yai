# RUNBOOK

1. Start runtime:
   - `yai up --ws demo-d1 --detach --allow-degraded`
2. Readiness checks:
   - `yai root ping`
   - `yai kernel --arming --role operator ping`
3. Execute one D1 trial:
   - `cd docs/40-qualification/RT-0.1-001-D1-EGRESS-CURL`
   - `BASELINE_ID=baseline-deny TARGET_PROFILE=local ./run/run-three.sh`
4. Capture runtime evidence root from terminal output (`~/.yai/qualifications/...` if used by wrapper).
5. Stop runtime (optional):
   - `yai down --ws demo-d1 --force`
