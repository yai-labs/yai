# 40 Qualification

Executable qualification program.

Purpose:
- Convert catalog scenarios into deterministic test gates with repeatable evidence.

Gate directories:
- `docs/40-qualification/QT-0.1-001-SC102/`
- `docs/40-qualification/QT-0.1-002-SC103/`
- `docs/40-qualification/RT-0.1-001-D1-EGRESS-CURL/`
- `docs/40-qualification/RT-0.1-002-D1-EGRESS-OTEL/`
- `docs/40-qualification/RT-0.1-003-D1-EGRESS-S3/`
- `docs/40-qualification/RT-0.1-001-D8-PARAMS-LOCK/`

Rule:
- Qualification artifacts define execution and evidence.
- Audits/proof validation remain under `docs/50-validation/`.

Execution families:
- `QT-*` = Scenario qualification gates (can be sim/live depending on gate design).
- `RT-*` = RealTarget trials (live-only by contract).
