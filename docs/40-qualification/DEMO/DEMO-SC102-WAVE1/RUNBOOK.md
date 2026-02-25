# RUNBOOK

1. Prepare runtime and capture base identity.
2. Execute SC102 wave runner:
   - `cd docs/40-qualification/QT-0.1-003-SC102-WAVE1`
   - `./run/run-wave.sh`
3. Capture generated `wave_id`, runtime output root, and bundle directory.
4. Open bundle:
   - `docs/40-qualification/WAVES/<wave_id>/README.md`
   - `docs/40-qualification/WAVES/<wave_id>/INDEX.md`
5. Run bundle verify:
   - `./verify/verify.sh`
6. Record operator summary in `artifacts/cli/operator-notes.txt`.

Expected outcomes:
- Wave runner exits `0`.
- Bundle verify exits `0`.
- Bundle contains selected D1 and D8 run evidence.
