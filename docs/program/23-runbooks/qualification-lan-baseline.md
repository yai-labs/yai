# Qualification LAN Baseline Runbook (QW-1)

## Goal

Run the first qualification wave in LAN/trusted-network mode with canonical command grammar and reusable test layout.

## Prerequisites

- Repo: `yai`
- Binaries buildable: `yai`, `yai-daemon`
- Local host supports UNIX sockets and standard temp directories

Build once:

```bash
make yai yai-daemon
```

## Qualification Layout

- `tests/integration/qualification/lib/`
- `tests/integration/qualification/fixtures/bologna-mini/`
- `tests/integration/qualification/lan/`

LAN wave runner:

```bash
tests/integration/qualification/lan/run_qw1_lan_wave_v1.sh
```

## LAN Wave Tests

1. `ql_lan_enroll_attach_emit_v1.sh`
2. `ql_lan_three_peers_same_workspace_v1.sh`
3. `ql_lan_peer_offline_replay_v1.sh`
4. `ql_lan_distinct_assets_v1.sh`
5. `ql_lan_overlap_assets_v1.sh`
6. `ql_lan_backlog_drain_v1.sh`

Each script prints `<name>: ok` on success.

## Expected Outcomes

- owner ingest path accepts enroll/attach/emit/status baseline
- one workspace is fed by multiple peers (3-peer baseline)
- peer offline/replay path persists spool and drains after owner recovery
- distinct coverage shows no overlap pressure
- overlap scenario is visible in source read model/conflict surfaces
- backlog/retry signals remain visible and non-ambiguous

## Fixture Set

`tests/integration/qualification/fixtures/bologna-mini/` includes:

- `peer-a-performance/` (CSV)
- `peer-b-programmazione/` (CSV)
- `peer-c-documentale/` (PDF-like fixtures)

This fixture set is intentionally small but non-trivial and coverage-oriented.

## Troubleshooting

- If socket bind/listen fails, rerun with clean temp home and ensure no stale socket path.
- If a LAN script fails, rerun the single script first, then the full wave.
- Keep owner/peer logs from temp directories when debugging intermittent replay/backlog failures.

## Next Wave Boundary

After QW-1 passes, move to secure peering qualification (`QW-2`) reusing:

- command contract (`docs/developer/qualification-command-contract.md`)
- qualification layout
- helper layer
- fixture discipline

Do not rename command grammar between LAN and peering waves.
