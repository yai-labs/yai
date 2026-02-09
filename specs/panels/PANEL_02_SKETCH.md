# PANEL 01 — SKETCH PANEL

**Workspace → Pannello di Progettazione Iniziale**

---

## 0. OVERVIEW FUNZIONALE

Lo **Sketch Panel** è il punto di ingresso per la definizione del progetto all'interno del workspace.
È il pannello in cui l'utente:

* definisce obiettivi, contesto e vincoli
* imposta stack tecnologico e caratteristiche del sistema
* seleziona la complessità progettuale
* genera piani, task iniziali, roadmap e architetture preliminari
* produce diagrammi tecnici
* raccoglie la knowledge base iniziale del progetto

Tutto avviene tramite:

* **LLM ChatBar (persistente, a destra)**
* **PlannerAgent, ProjectAgent, Analyzer, KnowledgeAgent, Scanner**, ecc.

Il pannello produce gli asset fondamentali che alimentano successivamente:

* Code Panel
* Knowledge Panel
* Agents Workflow
* Logs Panel
* Preview Panel
* Git Panel

Lo Sketch Panel **definisce la forma iniziale del progetto**.

---

## 1. LAYOUT STRUTTURALE (UI)

### 1.1 Layout generico del Workspace (globale)

Indipendente dal pannello:

* **Bottom Fixed Downbar** → icone pannelli (1..11) stile DaVinci Resolve
* **TopBar globale** → controlli workspace (status, refresh, sync, info)
* **Right ChatBar (LLM)** → persistente, a tendina, multi-chat
* **Bottom Toggle Terminal** → terminal, debug console, output, ports (sopra la downbar)

### 1.2 Layout interno dello Sketch Panel

```
 ┌─────────────────────────────────────────────────────────────────────────┐
 │ TOPBAR: workspace status, sync, info                                    │
 ├────────┬────────────────────────────────────────────────────┬───────────┤
 │        │                                                    │           │
 │  FILE  │  [A] SKETCH TOOLBAR                                │           │
 │        ├────────────────────────────────────────────────────┤           │
 │  EXP   │                                                    │           │
 │  LOR   │  [B] BRIEF EDITOR                                  │   CHAT    │
 │  ER    │      (multiline, markdown sync)                    │   LLM     │
 │        ├────────────────────────────────────────────────────┤           │
 │  (VS   │                                                    │  (multi   │
 │  Code  │  [C] GENERATED TASKS & STEPS                       │   chat)   │
 │  like) │      (kanban + outline)                            │           │
 │        ├────────────────────────────────────────────────────┤  (persi   │
 │        │  [D] NOTEBOOK       │  [E] DIAGRAMS PREVIEW        │   stent)  │
 │        │  (markdown)         │  (mermaid/uml)               │           │
 │        │                     │                              │           │
 ├────────┴────────────────────────────────────────────────────┴───────────┤
 │ TERMINAL TOGGLE: [Terminal] [Debug Console] [Output] [Ports]            │
 ├─────────────────────────────────────────────────────────────────────────┤
 │ DOWNBAR: [①Sketch][②Preview][③Agents][④Logs]...[⑪Settings]            │
 └─────────────────────────────────────────────────────────────────────────┘
```

### 1.3 Elementi globali condivisi con tutti i pannelli

* **Chat LLM** → sempre visibile, multi-chat, unificata tra pannelli
* **Downbar** → navigazione tra i pannelli (icone stile DaVinci Resolve)
* **Terminal Toggle** → "Terminal / Debug Console / Output / Ports" (stile VS Code)
* **TopBar** → stato workspace, pulsanti di sincronizzazione
* **Left File Explorer** → file tree del workspace (stile VS Code)

---

## 2. COMPONENTI PRINCIPALI DEL PANNELLO

### 2.1 SKETCH TOOLBAR

Componenti:

* **Generate Plan**
* **Update Plan**
* **Auto-Refine** (richiama PlannerAgent + AnalyzerAgent in cascata)
* **Complexity Level [dropdown]**
  * Small (MVP)
  * Medium
  * Large
  * Enterprise
  * Custom (con valori numerici)
* **Languages / Framework selector**
* **Reset / Clear Draft**
* **View selector:**
  * Brief
  * Tasks
  * Architecture
  * Entities
  * Roadmap
  * Notes
  * Diagrams

### 2.2 PROJECT BRIEF EDITOR

Editor multilinea:

* testo libero
* sincronizzato con la chat
* il PlannerAgent lo legge come *fonte primaria*

Il brief viene continuamente raffinato dagli agenti (Planner, Project, Analyzer).

### 2.3 TASKS & STEPS (GENERATI DA AGENTI)

Struttura tipo *Kanban + Outline*.

Formato dati:

```json
[
  {
    "task_id": "T001",
    "title": "Setup project skeleton",
    "description": "Initialize folder structure...",
    "steps": [
      {"step_id": "S001", "desc": "Create src/"},
      {"step_id": "S002", "desc": "Define app schema"}
    ],
    "status": "pending_review"
  }
]
```

Funzionalità:

* generati da PlannerAgent + ProjectAgent
* modificabili via UI
* discutibili via chat
* approvabili/rifiutabili
* al termine → salvati in `tasks.json` del workspace

### 2.4 NOTEBOOK INTEGRATO

Editor Markdown:

* appunti veloci
* riflessioni tecniche
* TODOs locali
* integrazione futura con Jupyter-like cell

Genera `notes.md` nel workspace.

### 2.5 DIAGRAMS PREVIEW (MERMAID / UML)

Funzionalità:

* viewer integrato
* toggle **Preview / Source**
* esportazione PNG/SVG
* copy-to-clipboard (solo mermaid)
* diagrammi generati da:
  * PlannerAgent (architecture diagrams)
  * ProjectAgent (module maps)
  * Analyzer (flowcharts)
  * KnowledgeAgent (entity maps)

### 2.6 MULTI-CHAT SYSTEM (ESSENZIALE)

Ogni chat è isolata semanticamente:

* `chat_brief`
* `chat_specs`
* `chat_arch`
* `chat_feature_x`

Ogni chat ha:

* `chat_id`
* nome
* descrizione
* contesto associato (brief / tasks / arch / roadmap)

Il modello LLM è uno, il routing del contesto è interno.

---

## 3. PARAMETRI DI CONFIGURAZIONE DEL PANNELLO

### 3.1 COMPLEXITY LEVEL

Valore fondamentale.

Influenza:

* granularità del planning
* profondità del reasoning
* numero di task generati
* dimensione della knowledge iniziale
* quantità di inferenze del Planner
* consumo memoria embedding
* livello della documentazione tecnica prodotta

Backend:

```python
runtime.project_complexity = <value>
```

### 3.2 STACK TECNOLOGICO

Selettori:

* linguaggi
* framework
* database
* cloud providers
* environment

Backend salva in:

```
workspace/settings/stack.json
```

### 3.3 RISORSE (RAM/CPU/VRAM)

**NON** nel pannello Sketch.
Sono nella card "Create Workspace".

Lo Sketch Panel le legge ma non le imposta.

---

## 4. INTERAZIONI BACKEND (WS REQUESTS)

### 4.1 GENERATE PLAN

```
method: workflow.plan.generate
```

Payload:

```json
{
  "workspace_id": "...",
  "brief": "...",
  "complexity": "enterprise",
  "stack": {...}
}
```

### 4.2 UPDATE PLAN

```
method: workflow.plan.update
```

### 4.3 NOTES WRITE

```
method: workspace.notes.write
```

### 4.4 DIAGRAM GENERATION

```
method: diagram.mermaid.generate
```

### 4.5 TASK GENERATION

```
method: planner.tasks.generate
```

---

## 5. STATI DEL PANNELLO

* `empty`
* `editing`
* `generating`
* `pending_review`
* `confirmed`
* `synced`

---

## 6. DIPENDENZE

Internamente usa:

* **PlannerAgent**
* **ProjectAgent**
* **KnowledgeAgent**
* **AnalyzerAgent**
* **ScannerAgent**
* **EmbeddingEngine**
* **UnifiedChatEngine**
* **Workspace Session Context**

---

## 7. OUTPUT DEL PANNELLO

File generati in workspace:

* `plan.json`
* `tasks.json`
* `knowledge_seed.json`
* `diagrams.mmd`
* `notes.md`
* `summary.yml`

---

## 8. EDGE CASES

* brief vuoto → il planning non parte
* complexity assente → default "Medium"
* stack incompleto → planner procede con fallback
* diagrammi invalidi → preview non renderizzabile
* chat disattiva → impossibile generare plan
* workspace non inizializzato → reject WS requests

---

## 9. NOTE DI IMPLEMENTAZIONE

* usare stato locale + globale (workspace store)
* evitare re-render pesanti su tasks board
* integrare autosave (brief, notes)
* mantenere idempotenza nelle chiamate generate/update
* preview diagrammi deve supportare 3 librerie: Mermaid, UML textual, Graphviz

---

## 10. INTEGRAZIONE CON ALTRI PANNELLI

* **Code Panel** → riceve struttura iniziale del progetto
* **Knowledge Panel** → riceve seed‐knowledge strutturata
* **Agents Panel** → mostra quali agenti hanno contribuito
* **Preview Panel** → mostra lo stato finale del plan
* **Git Panel** → può inizializzare repo dal plan
* **Containers Panel** → configura ambienti runtime successivi