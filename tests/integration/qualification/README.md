# Qualification Integration Layout

QW-1 introduces the qualification layout and LAN baseline wave.

- `lan/`: executable qualification scripts for local/trusted network baseline.
- `peering/`: reserved for secure peering wave (QW-2).
- `scale/`: reserved for scale simulation wave.
- `realflow/`: reserved for real-flow qualification wave.
- `fixtures/`: shared test datasets and sample assets.
- `lib/`: shared helpers (`qualification_common.sh`, `qualification_asserts.sh`).

Entrypoint for QW-1 LAN wave:

- `tests/integration/qualification/lan/run_qw1_lan_wave_v1.sh`
