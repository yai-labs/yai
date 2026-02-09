PANEL_06_CODE.md
CODE PANEL — Editing, Diff, Patch & Codegen Orchestration
1. SCOPO DEL PANNELLO

Il Code Panel è l’ambiente di visualizzazione, editing strutturato, patching e generazione automatica del codice del workspace.

Obiettivi principali:

preview e editing rapido senza IDE esterno

visualizzazione differenze (diff viewer)

applicazione patch generate dagli agenti

integrazione diretta con CoderAgent, AnalyzerAgent e RefactorAgent

esecuzione automatizzata di operazioni di codegen tramite LLM

navigazione dei file e directory del workspace

apertura file, highlight, jump-to-definition (minimale)

collegamento con Knowledge e Logs (spiegazioni, cause errori, soluzioni)

Il Code Panel non sostituisce l’IDE, ma implementa:

operazioni atomiche e sicure sul codice

preview e conferma delle modifiche

strumenti intelligenti di debugging e refactoring

2. ELEMENTI UI FONDAMENTALI
2.1 File Explorer (sinistra — persistente)

Visualizza l’albero del workspace.

Funzioni:

click → apri file

context menu → rename, delete, new file/folder

sync con workspace FS

refresh automatico dopo codegen

filtri (solo codice, solo test, solo config ecc.)

ricerca file (regex / fuzzy)

2.2 Editor / Preview Area (centro)

Modalità:

Preview Read-Only (default)

Edit Mode (editing inline per modifiche piccole)

Diff Viewer (side-by-side)

Patch Candidate Viewer (modifiche generate dagli agenti)

LLM Inline Actions (richieste contestuali: explain, refactor, fix, inline comments)

Highlight:

line numbers

semantic highlighting (via LSP base opzionale)

syntax error popups (parser locale)

2.3 Code Actions Toolbar

Componenti:

Apply Patch

Reject Patch

Regenerate Patch

Ask LLM (context-aware)

Jump to Usage

Run Quick Fix

Extract Function / Extract Class

Show References

Toggle Diff / Toggle Preview

Save File

Open in External Editor (VSCode)

2.4 Bottom Terminal (globale)

Utile per:

lanciare comandi

mostrare test runner

mostrare errori runtime

eseguire patch automatiche

correlazione immediata con gli stack trace del Logs Panel

2.5 Right Chat LLM (persistente)

Il Code Panel è strettamente integrato con:

CoderAgent

RefactorAgent

AnalyzerAgent

Funzioni:

generazione file

refactor specifici

analisi semantica di errori

patch generation

patch validation

generazione tests

3. EVENTI UI → BACKEND

Elenco dei messaggi WebSocket inviati dal pannello:

workspace.fs.list
workspace.fs.read
workspace.fs.write
workspace.fs.delete
workspace.fs.create
workspace.fs.rename

code.patch.generate
code.patch.apply
code.patch.reject
code.fix.suggest
code.explain
code.refactor
code.search.symbols
code.search.references

analysis.code.run
validator.tests.run
logs.stream.subscribe  (per context debugging)
knowledge.ingest.code

4. EVENTI BACKEND → UI (PUSH)

Messaggi inviati dal backend:

workspace.fs.updated
workspace.fs.file_changed
code.patch.generated
code.patch.diff
code.analysis.result
code.refactor.result
code.fix.result
planner.update   (opzionale, se la modifica influisce sui task)
logs.stream.data
knowledge.item.created
timeline.event.created

5. INTEGRAZIONE CON ALTRI PANNELLI / AGENTI
5.1 Agenti coinvolti

CoderAgent → generazione codice, file, patch

RefactorAgent → refactoring strutturale

AnalyzerAgent → analisi semantica file e moduli

KnowledgeAgent → ingest codice → knowledge graph

HistorianAgent → timeline degli interventi

KnowledgeSyncAgent → sync refactor reports in Obsidian

ProjectAgent → aggiornamento scaffolding generale

5.2 Collegamenti con i pannelli

Logs Panel → mostra errori generati dal codice (runtime/test)

Knowledge Panel → ingest automatico del codice modificato

Agents Panel → mostra attività di Coder/Refactor/Analyzer

Preview Panel → mostra versione sintetica del progetto e quick refactor

Git Panel → commit patch generate automaticamente

Workflow / Timeline → ogni patch genera snapshot

Diagramma sintetico:

[Code Panel] → CoderAgent → (patch, file)
[Code Panel] → AnalyzerAgent → (metrics, hotspots)
[Code Panel] → RefactorAgent → (refactor suggestions)
[Code Panel] → KnowledgeAgent → (ingest)
[Code Panel] → Git Panel → (commit diff)

6. STATO GESTITO DAL PANNELLO
6.1 Stato locale (frontend)

file attualmente aperto

modalità visualizzazione (preview/edit/diff/patch)

patch candidate

diff attivo

posizione cursore

stato dell’editor (dirty/clean)

ultimo comando LLM applicato

6.2 Stato globale del workspace

file system virtuale

indice simboli (quando disponibile)

patch history

refactor history

code analysis cache

7. TASTI, AZIONI, SHORTCUTS

Suggeriti (stile VSCode):

CTRL+S → salva

CTRL+P → quick open file

CTRL+SHIFT+P → AI command palette

CTRL+D → toggle diff

CTRL+SHIFT+R → refactor suggestions

F12 → go to definition

F2 → rename symbol

ALT+ENTER → quick fix

SHIFT+ALT+F → format

8. OUTPUT GENERATI DAL PANNELLO

File generati/salvati nel workspace:

/src/... (modifiche ai file)

/patches/*.diff

/analysis/code_metrics.json

/refactor/reports/*.md

/knowledge/code_items/*.json

/timeline/events/*.json

9. CASI LIMITE / EDGE CASES

file molto grandi → rendering chunked

patch conflittuali → merge failure

codice non parsabile → fallback raw diff

modifiche LLM erronee → safe preview required

scrittura file negata → permission denied

disallineamento workspace FS → sync necessario

codegen troppo ampio → richiesta conferma utente

race conditions tra patch multiple → apply queue

10. NOTE DI IMPLEMENTAZIONE

Editor basato su <pre> shadow oppure Monaco (preferibile)

Utilizzare WebSocket streaming per patch grandi

Diff viewer side-by-side con syntax highlight differenziale

Prevedere auto-formatting e linting (opzionale)

Prevedere un log locale delle operazioni dell’LLM

Ogni patch deve essere reversibile (undo, rollback)

Isolare la generazione file in directory temporanea prima dell’apply

Supportare nativamente Python, JS/TS, Rust, Go, Java, PHP, C#

FINE DOCUMENTO