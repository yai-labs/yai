---
id: D-MAJOR
title: Macro Domains (D-Major) - Cross-Domain Coherence
status: active
owner: catalog
effective_date: 2026-02-23
revision: 1
---

# YAI - Macro Domains (D-Major)

This document defines the eight macro domains used to drive YAI's cross-domain coherence.

The purpose of D-Major is simple: prove that YAI is an infrastructure layer, not a single vertical tool.
YAI must preserve a single runtime grammar across domains:

trigger -> context -> authority/contract -> decision -> enforcement -> evidence

What changes across domains is the semantics, captured through Domain Packs.

## D1) Digital Domain (Compute & Information)

Everything natively made of bits: compute, storage, networking, protocols, distributed systems, software supply chain, APIs, runtimes.

Typical events:
- message ingress, RPC frames, state commits
- network connections, provider calls
- filesystem reads/writes, database queries
- build/release actions, dependency changes

Why it matters:
This domain is the closest to the runtime's native control surface. It is typically the first domain used for qualification (e.g., SC-102 core-only).

## D2) Physical Domain (Matter, Energy, Space-Time)

Systems where actions move matter/energy and are constrained by physics: sensors/actuators, cyber-physical systems, vehicles, robotics, industrial control, energy, avionics/space.

Typical events:
- telemetry and sensor readings
- actuator commands, safety interlocks
- physical faults, degradation, environmental latency

Why it matters:
Enforcement can have immediate real-world consequences. Domain semantics must capture safety windows, interlocks, and fail-closed boundaries.

## D3) Biological Domain (Life)

Clinical, biotech, wet-lab processes, bioinformatics, ecosystems.

Typical events:
- measurements, lab protocols, sample handling
- consent capture, chain-of-custody updates
- results, transformations, dataset access

Why it matters:
Authority and evidence must support consent and chain-of-custody constraints, not just access control.

## D4) Social-Institutional Domain (People & Institutions)

People, institutions, and procedures: law, public administration, governance, regulation, education, labor, welfare.

Typical events:
- formal acts, procedural steps, approvals
- access requests, delegations, notifications
- disputes, accountability events

Why it matters:
The semantics are procedural: a decision is not only allowed/denied but must be tied to roles, duties, and procedural legitimacy.

## D5) Economic Domain (Value & Exchange)

Finance, credit, payments, insurance, contracts, procurement, markets.

Typical events:
- authorization/denial, scoring, holds/releases
- transactions, settlement steps, contractual conditions
- disputes and reversals

Why it matters:
This domain is strict about limits, counterparties, and contractual scope. Decisions must be stable and explainable, and evidence must support audit and dispute resolution.

## D6) Operational Domain (Coordination & Logistics)

Coordination of complex systems: logistics, planning, maintenance, incident response, SRE/ops, emergencies.

Typical events:
- scheduling, prioritization, allocation
- rerouting, failover, recovery actions
- escalation and post-incident evidence

Why it matters:
This is where time-to-action becomes money. Domain packs must define what actions require arming/escalation and what can be automated safely.

## D7) Cognitive-Cultural Domain (Meaning & Media)

Meaning, content, and cultural dynamics: media, communication, reputation, moderation, knowledge operations.

Typical events:
- publication/visibility changes, ranking decisions
- enforcement actions, access restrictions
- provenance checks, appeals/contestation

Why it matters:
Semantics revolve around provenance, policy compliance, and contestability. Evidence must make decisions auditable and explainable.

## D8) Scientific Domain (Knowledge & Measurement)

Science as a chain of measurements and transformations: experiments, simulations, pipelines, reproducibility.

Typical events:
- data acquisition, preprocessing, exclusions
- parameter declarations, transformations
- results publication with evidence

Why it matters:
Reproducibility is a core constraint. Domain packs must define parameter locks, provenance, and evidence that supports repeatability.

## Cross-Domain Coherence Note (SC-102 alignment)

SC-102 (core-only qualification) should demonstrate that the following invariants remain identical across all D-Major domains:

- authority envelope integrity
- policy/contract chain (baseline hashable)
- decision vocabulary stability (allow/deny/quarantine/rate_limit)
- fail-closed behavior for forbidden effects
- evidence pack completeness and replayability
- minimal disclosure (only necessary information emitted)

Only the domain semantics should change (via Domain Packs), not the runtime.

## Domain Packs (reference)

Domain packs are defined under:
`docs/40-catalog/domains/packs/<Dk>/<pack-id>/`

Each pack should declare:
- forbidden effects (domain semantics)
- expected reason codes
- required evidence fields and KPIs
- baselines (allow/deny/quarantine)
- safe vectors (stimuli) and expected outcomes
