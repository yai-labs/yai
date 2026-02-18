# tools/ops/suite/ops

Operational scenario suites.

## Contents

- `no-llm-360.sh`: full operational scenario without active LLM dependency.
- `perf-slo-v1.sh`: latency and p95 SLO checks.
- `fault-injection-v1.sh`: process fault resilience checks.
- `security-sanity-v1.sh`: invalid-input robustness checks.
- `recovery-compat-v1.sh`: restart/recovery compatibility checks.
- `stress-v1.sh`: repeated workspace stress cycles.
