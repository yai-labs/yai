---
id: SC102-DEPLOYMENT-NOTES-L1
title: SC102 L1 Remote Controlled Deployment Notes
status: draft
owner: runtime
effective_date: 2026-02-24
revision: 1
---

# SC102 L1 Remote Controlled Deployment Notes

## Endpoints
- `curl.<your-domain>/`
- `otel.<your-domain>/v1/traces`
- `s3.<your-domain>/bucket/object`

## Controls
- DNS + TLS termination under your control.
- Correlate by `ws_id`/`trace_id` from evidence.
- Keep endpoint behavior deterministic (`200` on precheck).

## Expected behavior
- `baseline-deny`: deny + blocked + no exfil.
- `baseline-allow` (future extension): allow only contracted destinations.
