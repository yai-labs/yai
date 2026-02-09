PANEL_05_LOGS.md
LOGS PANEL
Osservabilità unificata del Workspace e del Runtime di ICE Studio
1. OBIETTIVO DEL PANNELLO

Il Logs Panel fornisce un'unica vista centralizzata di tutti i log generati:

dal runtime di ICE Studio

dagli agenti

dal backend Python

dal frontend Electron/JS

dai debugger attivi

dalla pipelines di Knowledge e RAG

dal workspace utente (file di log del progetto, stdout/stderr, crash, test runner)

dai backend esterni (DB relazionali, VectorDB, cache)

Il pannello è progettato come sistema di osservabilità professionale, orientato a debugging, tracing, correlazioni timeline, e analisi automatizzata tramite LLM.

2. STRUTTURA GENERALE

Il Logs Panel è suddiviso in sei macro-sezioni:

Live Log Stream (runtime interno)

Stack Trace Viewer (stack navigabili)

Workspace Log Viewer (file di log del progetto utente)

Event Bus Viewer (eventi di orchestrazione)

Debugger Plugins (debug multipiattaforma)

LLM Debug Console (assistenza intelligente)

Ogni sezione utilizza filtri, ricerca, severità, grouping, ed è totalmente sincronizzata con Timeline, Knowledge e Agents.

3. LIVE LOG STREAM (RUNTIME DI CORTEX)
3.1 Sorgenti del Live Stream

Colleziona e mostra in tempo reale:

backend/python.log*

frontend/electron-main.log

console del renderer (JS)

WebSocket server (ws_server)

orchestrator/dispatcher/router

middleware (auth/logging/tracing)

event_bus

storage (sqlite, postgres, mysql, mariadb, duckdb)

vector DB (faiss/chromadb)

LLM engine

rag pipeline

knowledge ingestion

embedding pipeline

agent runner + scheduler

3.2 Funzionalità del Live Stream

modalità follow tail (stile tail -f)

severity highlighting

filtri per componente, modulo, agente, tipo di errore

regex search

buffering e replay dello stream

collapsible stack traces

click → jump to Timeline Event

click → open related task in Agents Panel

export in file

4. STACK TRACE VIEWER

Una vista professionale per stack trace strutturati e navigabili.

4.1 Tipi di Stack Trace

ICE Internal Stack (orchestrator, router, dispatcher, ws_server)

Agent Stack (task, errori, failure, timeout)

RAG Pipeline Stack

KnowledgeEngine Stack

Storage/DB Stack

User Application Stack (errori del progetto)

4.2 Funzioni

grouping per agente o componente

apertura file sorgente nel Code Panel

navigazione call-chain

filtri per exception type e modulo

integrazione con LLM:

explain stack

propose fix

correlate with Knowledge

integrazione Timeline:

attach to timeline snapshot

5. WORKSPACE LOG VIEWER (LOG DEL PROGETTO UTENTE)

Questa sezione gestisce i log del workspace:

5.1 Sorgenti

file .log del progetto (nginx, apache, syslog, app.log, ecc.)

stdout/stderr dei processi lanciati

terminali (VSCode terminal bridge)

test runner (pytest, jest, go test)

file rotanti generati dal progetto

crash report, core dump leggeri

5.2 Funzioni

file explorer integrato

stream per file grandi (chunked rendering)

regex, date-range, severity

auto highlighting error/warnings

log normalization via LogAgent

correlazione con Knowledge → log ingestion

correlazione con Timeline → eventi associati

analisi semantica con LLM

6. EVENT BUS VIEWER

Mostra la sequenza di eventi dell’orchestrazione interna.

6.1 Eventi tipici

AgentExecutionStarted / Finished

AgentErrorRaised

OrchestratorCall

Dispatch / Routing

StorageQuery / StorageError

GraphUpdated / RAGQueryExecuted

EmbeddingsGenerated

FileSaved

WorkspaceSwitched

TimelineEventCreated

6.2 Funzioni

filtri per categoria

correlazione con Live Stream

click → apri stack

click → apri Knowledge item

correlazione timeline + knowledge graph

7. DEBUGGER PLUGINS (MODULARI)

Il Logs Panel può attivare debugger multipli.

7.1 Debugger backend supportati

Python (pdb, debugpy)

NodeJS (inspector)

Go (delve)

Java (jdb)

Rust (lldb)

Docker container log tailing

Kubernetes pod log tailing

process inspector (psutil)

7.2 Debugger frontend supportati

Chrome DevTools Protocol

Renderer console monitor

WebSocket inspector

React DevTools (se attivato)

7.3 Output catturati

stack trace

breakpoints events

variable dump (quando supportato)

stdout/stderr

eccezioni di runtime

call frames e scopes

8. LLM DEBUG CONSOLE (ASSISTENTE INTELLIGENTE)

Pannello laterale sempre disponibile, integrato con Agents e Knowledge.

8.1 Funzioni principali

explain error / explain stack

root cause analysis

propose fix steps

highlight suspicious lines

generate patch code

run semantic search across logs

cluster log patterns (ML + LLM)

extract anomalies

estrazione e normalizzazione dei log

trasformazione dei log in knowledge items

sync diretto al KnowledgeSyncAgent

9. INTEGRAZIONE CON ALTRI PANNELLI
9.1 Agents Panel

visualizza log per-agente

il Logs Panel mostra vista unificata

cross-link: task → logs, logs → agent panel

9.2 Knowledge Panel

log ingestion automatica

salvataggio stack trace nel vault

correlazione semantica dei log

9.3 Code Panel

open file alla riga dello stack

generazione patch automatica

9.4 Timeline Panel (interno al Knowledge)

ogni log può generare timeline event

correlazione tempo reale log ↔ snapshot

9.5 Analysis Panel

Distinzione chiara:

Analysis Panel = performance, metriche, benchmark, test correctness
Logs Panel = debugging, stack trace, errori, warnings, tracing operativo

10. DATA MODEL LOGS (BACKEND)
10.1 Log Entry Standard

Ogni log entry deve essere strutturata:

{
  "timestamp": "...",
  "severity": "INFO|WARN|ERROR|DEBUG",
  "component": "agent|orchestrator|router|ws|storage|vector|knowledge|rag|frontend|backend|user",
  "agent": "optional",
  "workspace_id": "...",
  "message": "...",
  "stack": "...",
  "metadata": {...}
}

11. PIPELINE LOGS → KNOWLEDGE

Ogni log può diventare:

knowledge item

timeline event

embedding entry

cluster semantico

refactor suggestion

anomaly flag

Gestito tramite:

KnowledgeAgent

KnowledgeSyncAgent

HistorianAgent

12. OPERAZIONI DISPONIBILI (LLM + UI)

follow/unfollow stream

filter by module/agent/component

search regex

semantic search (LLM)

export

attach to timeline

ingest to knowledge

open in code panel

generate fixes

highlight anomalies

cluster messages simili

jump to origin (file/line)

13. NOTE DI IMPLEMENTAZIONE (FRONTEND)
13.1 Componenti principali

LogsPanelRoot.html/.js

LogsStream.html/.js

StackTracePanel.html/.js

LogsFilters.html/.js

renderLogs.js

CSS dedicato (logs.css)

13.2 IPC endpoints

File: ipc-logs.js

Endpoint:

logs.stream.subscribe

logs.stream.unsubscribe

logs.get_file_listing

logs.read_file_chunk

logs.get_stack_traces

logs.get_recent_events

logs.debugger.start

logs.debugger.stop

logs.search.regex

logs.search.semantic

14. NOTE DI IMPLEMENTAZIONE (BACKEND PYTHON)
14.1 Moduli sorgenti

cortex.logging_config

cortex.backend.main

cortex.backend.ws_server

cortex.storage.*

cortex.agents.*

cortex.orchestrator.*

cortex.knowledge.*

cortex.rag.*

14.2 Eventi WebSocket

push su logs.stream

push stack trace su logs.stack

push event bus su logs.events

15. FUTURE EXTENSIONS

OpenTelemetry exporter

JSON logs ingestion API

Remote logs tailing (SSH, containers, pods)

Timeline-based replay

Session diff di log

FINE DOCUMENTO