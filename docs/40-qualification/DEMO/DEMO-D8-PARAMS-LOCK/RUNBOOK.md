# RUNBOOK

1. Start runtime:
   - `yai up --ws demo-d8 --detach --allow-degraded`
2. Readiness checks:
   - `yai root ping`
   - `yai kernel --arming --role operator ping`
3. Start artifact store profile (docker path):
   - `cd docs/40-qualification/RT-0.1-001-D8-PARAMS-LOCK`
   - `BASELINE_ID=baseline-deny TARGET_PROFILE=docker ./run/run-three.sh`
4. Confirm no forbidden outputs persisted.
5. Stop runtime if needed:
   - `yai down --ws demo-d8 --force`
