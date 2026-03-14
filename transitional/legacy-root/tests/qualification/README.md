# Qualification Integration Layout

QW-1 introduces the qualification layout and LAN baseline wave.

- `lan/`: canonical qualification lane (container/system-centered future scenarios).
- `peering/`: reserved for secure peering wave (QW-2).
- `scale/`: reserved for scale simulation wave.
- `realflow/`: reserved for real-flow qualification wave.
- `fixtures/`: shared test datasets and sample assets.
- `lib/`: shared helpers (`qualification_common.sh`, `qualification_asserts.sh`).

Entrypoint for QW-1 LAN wave:

- legacy workspace-centered wave moved to:
  `tests/legacy/qualification/lan/run_lan_wave.sh`

QW-1 workspace-centered scenarios are fenced as legacy and no longer define
the canonical qualification path.

Evidence output:

- default root: `tests/qualification/evidence/<run-id>/`
- override with `QW1_EVIDENCE_ROOT=/abs/path`
- metadata: `meta.txt`
- per-step logs: `<script>.log`
- execution summary: `results.tsv`
