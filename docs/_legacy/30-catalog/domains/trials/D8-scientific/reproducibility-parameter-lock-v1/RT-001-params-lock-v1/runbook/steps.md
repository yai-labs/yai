# RT-001 Params Lock Steps

1. Start governed runtime context for trial execution.
2. Build run context (`ws_id`, `trace_id`, principal, baseline pin).
3. Prepare trial variant:
   - run-001: lock missing
   - run-002: lock hash mismatch
   - run-003: lock invalid
4. Attempt scientific run publish into `artifacts/run-outputs/`.
5. Enforce deny and abort publish when lock contract is violated.
6. Collect evidence bundle and assert pass/fail.
