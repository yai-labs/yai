---
id: RT-002
title: D1 Egress RealTarget Trial - OTEL export egress v1
status: draft
owner: catalog
effective_date: 2026-02-24
revision: 1
domain_pack_id: D1-digital/egress-v1
---

# RT-002-otel-export-egress-v1

## Purpose
Prove egress governance blocks an OTEL-style export path to a reachable local collector endpoint.

## Target
- Real tool: `curl` (OTLP/HTTP style request)
- Endpoint: `http://127.0.0.1:8443/v1/traces`

## Qualification runner
- `docs/40-qualification/RT-0.1-002-D1-EGRESS-OTEL/`
