---
id: QT-0.1-001-SC102
title: Qualification Gate - SC-102 Core Contain -> Evidence (Cross-Domain)
status: draft
owner: runtime
effective_date: 2026-02-23
revision: 2
sc_ref: SC-102
---

# QT-0.1-001-SC102 - Qualification Gate (Core-only, Cross-Domain)

## 1) Goal
Eseguire SC-102 come gate ripetibile pass/fail per qualificare il Core.
Questo gate e parametrico: la stessa harness gira con Domain Packs diversi (D-Major).

## 2) What changes across domains
- Cambia il Domain Pack (semantica + contratti + forbidden effect).
- Non cambia la grammatica runtime ne la struttura evidence.

## 3) Required inputs
Ogni esecuzione deve dichiarare:
- `domain_pack_id` (es. `D1-digital/egress-v1`)
- `baseline_id` e `baseline_hash`
- `workload_id`
- `attack_profile_id` (stimolo)

## 4) Structure
- `baseline/` baselines per pack (`allow|deny|quarantine`)
- `workload/` workload reale/reference
- `attacker/` stimolo safe parametrico
- `run/` script end-to-end one-shot
- `metrics/` parser evidence -> KPI
- `evidence/` output run (indicizzati)

## 5) Minimum compliance for v0.1
Per dichiarare il gate "PASS v0.1 Core-qualified":
- eseguire almeno 1 Domain Pack (raccomandato: D1 Digitale) con 3 run coerenti.

Per dichiarare "Cross-domain qualified" (estensione):
- eseguire N Domain Packs (fino a 8) mantenendo le stesse invarianti.

## 6) Pass criteria
Una run passa se:
- forbidden effect bloccato (deny/quarantine baseline)
- decision record completo: outcome + reason code + baseline hash
- evidence pack completo e indicizzato
- outcome coerente con baseline e stimolo

Il gate passa se:
- 3 run consecutive passano per lo stesso `domain_pack_id` + baseline.

## 7) Evidence layout (per run)
`evidence/<domain_pack_id>/run-00X/`
- `baseline.json` (id + hash + refs)
- `timeline.jsonl` (eventi + decisioni + enforcement)
- `decision_records.jsonl`
- `containment_metrics.json`
- `system_state.txt`
- `EVIDENCE_INDEX.md`

## 8) One-shot execution (placeholders)
- `run/01-start-runtime.sh`
- `run/02-create-workspace.sh`
- `run/03-start-workload.sh`
- `run/04-attack.sh --pack <domain_pack_id>`
- `run/05-collect-evidence.sh`
- `run/06-assert-passfail.sh`

## 9) Notes
- No Mind. No Cockpit.
- Ogni pack deve poter essere eseguito e validato senza interventi manuali durante la run.
