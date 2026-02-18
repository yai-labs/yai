# Toolchain Contract v1 (YAI)
Effective: 2026-02-18
Status: Active
Owner: maintainer

## Perché esiste
Questo repo non accetta “PR a sentimento”.
Ogni cambiamento deve essere tracciabile, reviewabile, ripetibile.

## Regola 1 — Branch
1) Di default: ogni branch nasce da una Issue.
2) Eccezione ammessa (META): lavori su governance/tooling/docs-infrastruttura possono nascere senza Issue.
   In quel caso la PR DEVE dichiarare `Issue-ID: N/A` e `Classification: META`.

Naming consigliato:
- feat/<issue-id>-<slug>
- fix/<issue-id>-<slug>
- chore/<issue-id>-<slug>
- meta/<slug>

## Regola 2 — PR
- La PR si apre DOPO che esiste un branch con commit pushati.
- La PR deve usare un template e avere metadata minimi (vedi “PR Metadata Minimum”).
- Merge: solo manuale dal maintainer. Gli agenti possono fare solo commit/push.

## Regola 3 — PR Metadata Minimum (obbligatorio)
Ogni PR deve contenere queste righe nel body:

- Issue-ID: <#123> oppure N/A
- Base-Commit: <40-char-sha>
- Classification: <FEATURE|FIX|DOCS|OPS|META>
- Compatibility: <A|B|C>
- Evidence:
  - Positive: ...
  - Negative: ...

## Regola 4 — Come scegliere template PR (UI o gh)
UI:
- usa la pagina di creazione PR con query param `template=<file.md>`.

CLI (gh):
- `gh pr create --template .github/PULL_REQUEST_TEMPLATE/<file.md>`

## Regola 5 — PR body “compilato”
Usiamo il tool:
- `tools/bin/yai-pr-body --template docs-governance --issue 123`
Questo genera `.pr/PR_BODY.md` con Base-Commit e placeholder pronti.

## Regola 6 — Enforcement
Una GitHub Action rifiuta PR che non rispettano i metadata minimi.
