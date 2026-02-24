# Runbook Steps (RT-001)

1. Start local mock endpoint on `127.0.0.1:8443`.
2. Run curl precheck to prove endpoint reachability.
3. Start governed runtime (`yai-root-server`, `yai-engine`).
4. Send egress request through governed runtime path.
5. Verify deny decision reason `EGRESS_DEST_NOT_CONTRACTED`.
6. Collect and normalize evidence bundle.
7. Repeat 3 consecutive runs.
