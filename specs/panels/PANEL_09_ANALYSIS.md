PANEL_09_ANALYSIS.md
ANALYSIS PANEL — Code, Logs, Metrics, Performance, ML Insights, Tests, Hotspots
1. SCOPO DEL PANNELLO

Il Analysis Panel fornisce una vista unificata di tutte le analisi tecniche generate dagli agenti di Cortex:

AnalyzerAgent → analisi statica codice, complessità, hotspot, dead code

MLAgent → anomaly detection, clustering, statistical patterns

ValidatorAgent → test discovery, test run, coverage

LogAgent → log patterns, error correlation

ScannerAgent → project scan, dependency map, unresolved refs

HistorianAgent → timeline tecnica con eventi di esecuzione

CodeAgent → diffs e patch metadata

ProjectAgent → report di generazione progetto

KnowledgeSyncAgent → esportazione analisi in Obsidian

Il pannello permette di identificare problemi, correlazioni, regressioni e migliorare la qualità del progetto.

Funziona come:

dashboard tecnica

performance monitor

debugging advanced

hotspot explorer

ML insights explorer

2. ELEMENTI UI FONDAMENTALI

L’Analysis Panel è composto da 5 macro-sezioni, tutte switchabili dalla toolbar interna.

2.1 Analysis Toolbar (header)

Contiene:

refresh analysis

run full analysis

run selective analysis:

code

logs

tests

performance

ML

filter builder:

files / modules

error category

type: code / logs / tests / ml

AI Assist:

explain results

correlate errors

propose fixes

rank severity

2.2 Section A — CODE ANALYSIS VIEW

Tre componenti:

(1) Code Hotspot Map

Heatmap dei file più problematici basata su:

numero errori

complessità

duplicazioni

static analysis score

refactor suggestions

(2) Code Metrics

Pannello a tabelle + grafici:

LOC

cyclomatic complexity

maintainability index

duplicated blocks

dependency depth

unused imports

unreachable code

Supporta:

drill-down per file

export chart

AI “Explain metric”

(3) Code Issue List (tipo ESLint / SonarQube)

Ogni issue con:

file

line

extractor (AnalyzerAgent)

severity

reasoning LLM (optional)

fix suggestion (RefactorAgent)

2.3 Section B — LOG ANALYSIS VIEW

Diversa dal Logs Panel:

qui non c’è streaming

ci sono analisi e correlazioni statiche aggregate

Componenti:

log summary

log frequency chart

error types distribution

cluster anomalie (MLAgent)

correlazioni tra errori e commit (HistorianAgent)

AI “Root cause analysis”

2.4 Section C — PERFORMANCE ANALYSIS VIEW

Metriche:

CPU usage trend

Memory usage

VRAM usage (se MCU/GPU enabled)

embedding throughput

inference times

vector search latency

DB (relational) query time

DB (vector) build/search time

Visualizzazione con:

line charts

bar charts

flame charts (per eventuali LLM → tokens)

waterfall execution chain (orchestrator event trace)

2.5 Section D — TEST RESULTS VIEW

Dati dal ValidatorAgent:

test discovery

test execution

failed tests

flaky test detection

code coverage

failure reasoning (LLM)

auto-fix proposal (RefactorAgent)

2.6 Section E — ML (Clustering / Anomaly Detection)

Da MLAgent:

clustering grafico (embedding metadata)

anomaly score list

correlated anomalies

cluster heatmaps

explanation textual (LLM)

2.7 Section F — TIMELINE (Technical Timeline)

Timeline tecnica:

scan events

analyzer events

validation runs

ML executions

performance peaks

incidents

Ogni evento cliccabile mostra:

analysis summary
raw payload
logs
related code
related tests
AI explanation
suggested fix

3. EVENTI UI → BACKEND
analysis.full.run
analysis.code.run
analysis.logs.run
analysis.tests.run
analysis.performance.run
analysis.ml.run

analysis.code.file
analysis.logs.query
analysis.tests.re-run
analysis.ml.cluster
analysis.ml.score
analysis.timeline.fetch

analysis.ai.explain
analysis.ai.correlate
analysis.ai.root_cause
analysis.ai.propose_fix

4. EVENTI BACKEND → UI
analysis.full.done
analysis.code.result
analysis.logs.result
analysis.tests.result
analysis.performance.result
analysis.ml.result
analysis.timeline.result

analysis.ai.explanation
analysis.ai.correlation
analysis.ai.fix
analysis.ai.root_cause

analysis.error

5. INTEGRAZIONE CON ALTRI PANNELLI / AGENTI
Agenti Coinvolti

AnalyzerAgent – static/code analysis

LogAgent – log patterns

ScannerAgent – project scan issues

ValidatorAgent – test results

HistorianAgent – technical timeline

MLAgent – clustering/anomalies

KnowledgeAgent – ingest dei file analizzati

KnowledgeSyncAgent – esportazione markdown

ProjectAgent – orchestration-level summary

CoderAgent / RefactorAgent – fix proposals

Collegamenti con altri pannelli
[Code Panel] → mostra file problematici
[Logs Panel] → raw logs da cui derivano le analisi
[Sketch Panel] → analisi iniziale della struttura
[Preview Panel] → summary tecnico del workspace
[Git Panel] → correlazione errori ↔ commits
[Knowledge Panel] → aggiornamento grafo e embedding

6. STATO GESTITO DAL PANNELLO
Locale (frontend)

selected section

selected filters

selected file/module

last analysis result

charts state

drill-down state

Globale (workspace)

dataset analitico persistente

storia delle analisi

timeline tecnica

ML state (clusters, anomalies)

snapshot relazioni logs ↔ code ↔ tests

7. TASTI / SHORTCUTS

CTRL+SHIFT+A → apri Analysis Panel

F5 → run full analysis

CTRL+ALT+C → code analysis

CTRL+ALT+L → log analysis

CTRL+ALT+T → test analysis

CTRL+ALT+P → performance

CTRL+ALT+M → ML clustering

F8 → AI explain selection

SHIFT+F8 → propose fix

8. OUTPUT GENERATI DAL PANNELLO

Produzione file nel workspace:

analysis/code/*.json

analysis/logs/*.json

analysis/tests/*.json

analysis/performance/*.json

analysis/ml/*.json

analysis/reports/*.md

timeline/analysis_events.json

knowledge/analysis_ingest/*.md

9. EDGE CASES

progetto vuoto → no analysis

logs insufficienti → fallback

ML clustering instabile → fallback a basic stats

test runner crash → suggestion to fix env

code complexity troppo alta → viewer degrade mode

performance sampling out-of-sync → resync orchestrator

timeline corrotta → fallback raw list

10. NOTE DI IMPLEMENTAZIONE

charts JS interni → PerformanceChartsPanel.js

ML panel usa worker thread per non bloccare UI

code metrics devono riusare AST già generato da AnalyzerAgent

timeline deve essere unificata con Logs Panel (event bus)

usare caching per i risultati più pesanti

LLM-based reasoning deve essere opzionale (toggle safe mode)

esportazione MD via KnowledgeSyncAgent già integrata

FINE DOCUMENTO