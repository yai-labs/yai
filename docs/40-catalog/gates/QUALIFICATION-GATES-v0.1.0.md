---
id: QUALIFICATION-GATES-v0.1.0
title: Qualification Gates - v0.1.0
status: draft
owner: program-delivery
effective_date: 2026-02-23
revision: 2
---

# Qualification Gates - v0.1.0

## Scopo
Definire i gate ufficiali (QT) per qualificare YAI v0.1.
I gate sono parametrizzabili per Domain Pack (D-Major): la grammatica runtime resta invariata.

## Gate ufficiali (v0.1)

| Gate (QT) | Scenario (SC) | D-Major coverage | Target | Domini runtime | Min evidence | Output |
|---|---|---|---|---|---|---|
| QT-0.1-001-SC102 | SC-102 | v0.1: almeno D1 (estendibile a D1..D8) | Core-qualified (Gate A) | control, workspace, providers/network/storage/resource, audit | 3 run coerenti per pack + KPI + evidence index | Evidence pack + metrics report |
| QT-0.1-002-SC103 | SC-103 | dipende da SC-102 + D-Major pack | Premium-qualified (Gate B) | + mind + agentic + rollout governance | 3 run coerenti + patch/PR evidence + rollout logs | Evidence pack Incident+Patch |

## Regole di qualificazione
- Core-qualified v0.1: QT-0.1-001-SC102 = PASS (almeno 1 pack, raccomandato D1).
- Cross-domain qualified: QT-0.1-001-SC102 = PASS su un set di pack D-Major (fino a 8).
- Premium-qualified v0.1: SC-102 PASS + SC-103 PASS.

## Note
Le dipendenze runbook/MP vanno tracciate nel Delivery Program (Wave plan) puntando a questi gate come target-state.
