# Runbook Steps (RT-002)

1. Start local OTEL-like endpoint on `127.0.0.1:8443`.
2. Execute OTLP/HTTP style precheck using curl POST `/v1/traces`.
3. Trigger governed runtime egress request representing exporter flow.
4. Verify deny with `EGRESS_DEST_NOT_CONTRACTED`.
5. Collect standard evidence bundle.
