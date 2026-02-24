---
id: DOMAIN-TRIALS
title: Domain RealTarget Trials - Catalog Layer
status: draft
owner: catalog
effective_date: 2026-02-24
revision: 1
---

# Domain RealTarget Trials

Trials define repeatable procedures that exercise real software targets under domain pack governance.

Rules:
- Packs remain semantic law (`forbidden_effects`, reason codes, baselines, vectors).
- Trials define real execution procedure and evidence extraction.
- Qualification runners execute trials and produce auditable bundles.

Layout:
- Catalog trial spec: `docs/30-catalog/domains/trials/<Dk>/<pack>/<RT-id>/`
- Qualification runner: `docs/40-qualification/RT-.../`

Current live trials:
- `D1-digital/egress-v1/RT-001-curl-egress-v1`
- `D1-digital/egress-v1/RT-002-otel-export-egress-v1`
- `D1-digital/egress-v1/RT-003-s3-upload-egress-v1`
