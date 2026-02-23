---
id: CATALOG-DOMAINS
title: Domains - Catalog Layer
status: draft
owner: catalog
effective_date: 2026-02-23
revision: 1
---

# Domains (Catalog)

This section defines the macro-domain taxonomy used by YAI to maintain cross-domain coherence.

The domains listed here are semantic domains (what the problem is about), not software modules.
They exist to prevent the platform from collapsing into one vertical product and to ensure that the same runtime grammar can be demonstrated across fundamentally different worlds (digital, physical, biological, etc.).

## What a Domain is (D-Major)

A D-Major domain is a macro-area of reality where events, authority, constraints, and consequences have a distinct meaning:

- Digital (compute/network/storage)
- Physical (cyber-physical, matter/energy)
- Biological (life, clinical/lab)
- Social/Institutional (people, regulation, procedures)
- Economic (value exchange, contracts)
- Operational (coordination, logistics, SRE/IR)
- Cognitive/Cultural (media, meaning, reputation)
- Scientific (measurement, reproducibility)

YAI does not become any of these domains. YAI remains a governed runtime.
What changes is the domain semantics captured by a Domain Pack.

## What remains invariant (the runtime grammar)

Across all domains, YAI must preserve the same chain:

trigger -> context -> authority/contract -> decision -> enforcement -> evidence

A domain changes what the event means and which effects are considered critical, but the grammar remains identical.

## Important distinction: Domain Semantics vs Runtime Effect Domains

- D-Major domains (this section) define semantics and validation criteria.
- Runtime effect domains (control, network, storage, providers, resource, etc.) define where the runtime can produce or block effects.

A D-Major domain can map to multiple runtime effect domains. Example:
- Economic domain semantics may involve provider calls (payments), storage (ledgers), and control (approval workflows).

## How domains are used in Catalog and Qualification

- Scenario Specs (SC-xxx) reference one or more D-Major domains.
- Qualification Gates (QT-xxx) execute scenarios with one or more Domain Packs.
- Domain Packs implement domain semantics: forbidden effects, expected reason codes, required evidence fields, and KPIs.

## Files

- `D-MAJOR.md` - canonical D-Major taxonomy.
- `packs/` - domain packs (semantics packages) referenced by scenarios and qualification gates.
