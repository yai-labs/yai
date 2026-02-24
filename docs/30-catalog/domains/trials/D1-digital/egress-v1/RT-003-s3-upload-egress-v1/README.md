---
id: RT-003
title: D1 Egress RealTarget Trial - S3 upload egress v1
status: draft
owner: catalog
effective_date: 2026-02-24
revision: 1
domain_pack_id: D1-digital/egress-v1
---

# RT-003-s3-upload-egress-v1

## Purpose
Prove egress governance blocks an S3-style upload path to a reachable local endpoint.

## Target
- Real tool: `curl` (`PUT` upload)
- Endpoint: `http://127.0.0.1:8443/bucket/object`

## Qualification runner
- `docs/40-qualification/RT-0.1-003-D1-EGRESS-S3/`
