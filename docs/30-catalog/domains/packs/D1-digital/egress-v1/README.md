# D1-digital/egress-v1 - Digital Domain Pack (Network Egress Control)

This pack defines the semantic meaning of forbidden outbound network egress in the Digital domain.

The runtime grammar must remain invariant:
trigger -> context -> authority/contract -> decision -> enforcement -> evidence

## Forbidden effect (semantic)
- A process inside a workspace attempts to establish an outbound connection to a destination that is not covered by an active contract/allowlist.

## Runtime effect domain (where enforcement happens)
- `network`

## Intended use
- SC-102 (core-only) qualification: prove fail-closed egress blocking + evidence completeness.

## Safety
Vectors use TEST-NET IP ranges (RFC 5737) to avoid real-world targets.

## RealTarget Trials
- Catalog trial spec: `docs/30-catalog/domains/trials/D1-digital/egress-v1/RT-001-curl-egress-v1/`
- Qualification runner: `docs/40-qualification/RT-0.1-001-D1-EGRESS-CURL/`
- Catalog trial spec: `docs/30-catalog/domains/trials/D1-digital/egress-v1/RT-002-otel-export-egress-v1/`
- Qualification runner: `docs/40-qualification/RT-0.1-002-D1-EGRESS-OTEL/`
- Catalog trial spec: `docs/30-catalog/domains/trials/D1-digital/egress-v1/RT-003-s3-upload-egress-v1/`
- Qualification runner: `docs/40-qualification/RT-0.1-003-D1-EGRESS-S3/`

Note:
- Pack files remain semantic law (contracts/reason codes/evidence schema).
- Trial files define executable real-target procedures and runtime-derived evidence.
