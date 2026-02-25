# DEMO Space

`DEMO` is the human-facing operational layer for SC102 demonstrations.

- `WAVE`: automated, deterministic, regression-oriented, bundle-producing.
- `DEMO`: operator-guided, live narration/capture oriented, credibility-oriented.

Canonical demos:
- `docs/40-qualification/DEMO/DEMO-SC102-WAVE1/`
- `docs/40-qualification/DEMO/DEMO-D1-EGRESS/`
- `docs/40-qualification/DEMO/DEMO-D8-PARAMS-LOCK/`

Evidence rule:
- Demo scripts must not write qualification evidence into versioned repo evidence folders.
- Qualification evidence remains under runtime path (`~/.yai/qualifications/...`) via QT/RT runners.
- Demo folders keep only operator artifacts (CLI outputs, logs, screenshot/video placeholders, manifest linkage).

Required linkage:
- `docs/30-catalog/scenarios/SC-102.md`
- relevant pack/trial refs under `docs/30-catalog/domains/`
- wave bundle refs under `docs/40-qualification/WAVES/`
