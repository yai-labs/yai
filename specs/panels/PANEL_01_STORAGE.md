# 💾 PANEL 01 — STORAGE PANEL

> **Workspace → Pannello Storage & Dataset Management**

---

## 📋 0. OVERVIEW FUNZIONALE

Lo **Storage Panel** è il pannello responsabile della gestione completa delle risorse persistenti del workspace.

### Responsabilità Principali

- 🗄️ Modellare e visualizzare **tutte le risorse persistenti del workspace**
- 💿 Gestire database, dataset, file system, media, asset esterni
- 🤖 Interrogare il database tramite LLM (query naturali → SQL generato)
- 📊 Visualizzare risultati in forma tabellare
- 🐍 Gestire ambienti virtuali (Python venv, Node env, etc.)
- 📦 Mostrare le dipendenze/librerie del progetto
- 🖼️ Mantenere materiali esterni (immagini, media, dataset)
- 📜 Fornire uno storico delle query/operazioni (via HistorianAgent)

### Posizionamento nel Workflow

Il pannello funge da **"punto dati"** del workspace, a metà tra:

- **Knowledge Panel** (che mostra concetti e grafi)
- **Logs Panel** (che mostra log di basso livello)
- **Code Panel** (che usa requirements/librerie)

### Importanza Strategica

È il pannello dove l'utente capisce:
- ✅ Cosa è contenuto nel workspace
- ⚖️ Cosa pesa e occupa spazio
- 💾 Cosa sta usando memoria
- 💬 Dove può "parlare con il database" tramite l'LLM

> ⚠️ **CRITICO:** Questo pannello è la base operativa prima dello Sketch Panel. **Senza uno storage configurato, il progetto non è avviabile.**

---

## 🖼️ 1. LAYOUT STRUTTURALE (UI)

### 1.1 Layout Globale Ereditato dal Workspace

Elementi indipendenti dal pannello:

| Elemento | Posizione | Descrizione |
|----------|-----------|-------------|
| **Chat LLM** | Right (persistente) | Query su DB, dataset, file |
| **Sidebar Workspace** | Left | File tree del progetto |
| **TopBar** | Top | Status storage, refresh DB, recalc index, info |
| **Bottom Downbar** | Bottom | Pannelli 1..11 |
| **Bottom Terminal Toggle** | Bottom (sopra downbar) | Terminale / debug / output / ports |

---

### 1.2 Layout Interno dello Storage Panel

```
 ┌─────────────────────────────────────────────────────────────────────────┐
 │ TOPBAR: Storage Status | DB Connected | Vector Connected | Usage Bars   │
 ├────────┬────────────────────────────────────────────────────┬───────────┤
 │        │                                                    │           │
 │  FILE  │  LEFT SIDEBAR (panel-local)                        │           │
 │        │   - Databases                                      │           │
 │  EXP   │      • Relational (SQLite/Postgres/MySQL/etc.)     │           │
 │  LOR   │      • Vector Index (FAISS/Chroma)                 │           │
 │  ER    │   - External Assets                                │   CHAT    │
 │        │      • Media / Images / Audio                      │   LLM     │
 │  (VS   │      • Datasets (CSV/JSONL/Logs/Parquet)           │           │
 │  Code  │   - Virtual Env                                    │  (multi   │
 │  like) │      • Python venv                                 │   chat)   │
 │        │      • Node modules                                │           │
 │        │      • Other envs                                  │  (persi   │
 │        ├────────────────────────────────────────────────────┤   stent)  │
 │        │                                                    │           │
 │        │  MAIN AREA                                         │  "Ask DB  │
 │        │                                                    │   Ask     │
 │        │  [A] DATABASE EXPLORER                             │   Storage │
 │        │  - Tables list                                     │   Ask     │
 │        │  - Query editor (LLM-assisted)                     │   Query"  │
 │        │  - Table preview (grid)                            │           │
 │        │  - Query history                                   │           │
 │        │                                                    │           │
 │        │  [B] DATASET / MEDIA BROWSER                       │           │
 │        │  - Visualizzazione list/grid                       │           │
 │        │  - Preview immagini/media                          │           │
 │        │                                                    │           │
 │        │  [C] VIRTUAL ENV & DEPENDENCIES                    │           │
 │        │  - Liste librerie                                  │           │
 │        │  - Check mismatch / missing libs                   │           │
 │        │  - "Fix requirements" (via LLM)                    │           │
 │        │                                                    │           │
 ├────────┴────────────────────────────────────────────────────┴───────────┤
 │ TERMINAL TOGGLE: [Terminal] [Debug Console] [Output] [Ports]            │
 ├─────────────────────────────────────────────────────────────────────────┤
 │ DOWNBAR: [①Storage][②Sketch][③Agents][④Logs]...[⑪Settings]            │
 └─────────────────────────────────────────────────────────────────────────┘
```

---

## 🧩 2. COMPONENTI PRINCIPALI DEL PANNELLO

### 2.1 DATABASE EXPLORER (Relational + Vector)

#### Contenuti Gestiti

**Tabelle Relazionali:**
- `agent_workflows`
- `knowledge_entities`
- `knowledge_relationships`
- `knowledge_embeddings`
- `llm_cache`
- Log/event tables

**Collezioni Vector:**
- FAISS indexes
- Chroma collections

#### Funzionalità Principali

| Funzione | Descrizione |
|----------|-------------|
| **Preview tabellare** | Visualizzazione dati in griglia |
| **Query editor** | SQL manuale o assistito da LLM |
| **Execution pipeline** | DB → Results → Grid |
| **Export** | CSV/JSON |
| **Refresh schema** | Aggiornamento struttura DB |

#### Query LLM - Esempi

```
💬 "Mostra le entità collegate al modulo planner"
💬 "Fammi vedere gli ultimi 20 workflow generati"
💬 "Trova le tabelle con più record"
```

**Flusso:** LLM → genera SQL → esecuzione → visualizzazione

---

### 2.2 QUERY HISTORY (via HistorianAgent)

Lo storico mantiene traccia di:

- 📝 Query SQL inviate
- 📊 Risultati parziali
- ❌ Errori DB
- ⏱️ Tempi di esecuzione
- 🔧 Modifiche allo schema
- 📥 Import/export

**Visualizzazione:** Timeline sincronizzata con Logs Panel

---

### 2.3 DATASET / MEDIA BROWSER

#### Cartelle Gestite

```
workspace/data/     → Dataset strutturati
workspace/assets/   → Asset del progetto
workspace/media/    → File multimediali
```

#### Funzionalità Complete

- 🖼️ Preview immagini
- 📊 Preview CSV/JSON
- 📤 Caricamento e rimozione file
- 🔄 Conversioni automatiche (CSV → parquet)
- 🏷️ Tagging dei dataset
- 🧠 Ingestion nel Knowledge Panel (via KnowledgeAgent)

#### Modalità di Visualizzazione

| Modalità | Descrizione |
|----------|-------------|
| **Grid (icone)** | Vista moderna a griglia |
| **List** | Stile file explorer |
| **Table** | Vista tabellare per dataset |

---

### 2.4 VIRTUAL ENV & DEPENDENCIES

#### Python Virtualenv

**Gestione:**
- 📄 `requirements.txt` (lettura e sync)
- 📦 Pacchetti installati
- ⚠️ Check mismatches:
  - Presenti nel progetto ma non installati
  - Installati ma non usati

#### Node.js

**Gestione:**
- 📄 `package.json`
- 📦 Dipendenze mancanti

#### Funzionalità Intelligenti

```
🔍 "Identify missing libs"
🔧 "Proponi fix" → LLM genera patch requirements.txt
📊 "Stima impatto librerie" avanzata
```

---

### 2.5 STORAGE STATUS OVERVIEW

#### Metriche e Barre di Stato

- 💿 Spazio su disco (workspace dir)
- 🗄️ Spazio DB
- 🧠 Dimensione vector index
- 📚 Numero record knowledge
- 🔄 Numero task/workflows
- 📦 Pesi media/dataset
- ⚠️ Eventuali errori storage

---

## ⚙️ 3. PARAMETRI DI CONFIGURAZIONE DEL PANNELLO

### 3.1 Storage Backends (per workspace)

Ogni workspace ha configurazione dedicata:

```yaml
relational: sqlite / postgres / mysql / mariadb / duckdb
vector: faiss / chroma / fake
cache: sqlite / memory / redis
```

> Il pannello deve riflettere questa configurazione dinamicamente.

---

### 3.2 Percorsi Storage

```
workspace/
├── data/              # Dataset strutturati
├── assets/            # Asset del progetto
├── media/             # File multimediali
├── venv/              # Virtual environment Python
├── node_modules/      # Dipendenze Node.js
├── *.db               # Database SQLite
└── vector_index/      # Indici vettoriali
```

---

### 3.3 Limiti di Spazio Configurabili (Opzionali)

- 📏 Max database size
- 📏 Max dataset size
- 📏 Max media size

---

## 🔌 4. INTERAZIONI BACKEND (WS REQUESTS)

### 4.1 Query SQL Assistita

```
method: db.query.sql
```

**Payload:**
```json
{
  "workspace_id": "...",
  "sql": "SELECT * FROM knowledge_entities LIMIT 50"
}
```

---

### 4.2 Query Naturale → SQL

```
method: db.query.llm
```

**Esempio:**
```
Input: "Mostra tutte le entità create negli ultimi 7 giorni"
Output: SELECT * FROM knowledge_entities WHERE created_at >= datetime('now', '-7 days')
```

---

### 4.3 Lista Tabelle

```
method: db.schema.list
```

---

### 4.4 Preview Dati

```
method: db.table.preview
```

---

### 4.5 Import Dataset

```
method: storage.dataset.import
```

---

### 4.6 Fix Requirements

```
method: storage.env.fix_requirements
```

---

## 🔄 5. STATI DEL PANNELLO

| Stato | Descrizione |
|-------|-------------|
| `empty` | Pannello vuoto, storage non configurato |
| `loading` | Caricamento dati in corso |
| `table_preview` | Visualizzazione tabella |
| `query_running` | Query in esecuzione |
| `error` | Errore operazione storage |
| `history_view` | Vista storico query |
| `env_analysis` | Analisi ambiente virtuale |
| `media_view_grid` | Vista griglia media |
| `media_view_list` | Vista lista media |

---

## 🔗 6. DIPENDENZE (AGENTI E MODULES)

### Agenti Utilizzati

| Agente | Ruolo |
|--------|-------|
| **KnowledgeAgent** | Ingestion dataset |
| **KnowledgeSyncAgent** | Sync verso graph |
| **HistorianAgent** | Query history |
| **AnalyzerAgent** | Analisi dataset |
| **PlannerAgent** | Suggerimenti su storage |
| **SystemAgent** | File ops, disk usage |

### Moduli Core

- **EmbeddingEngine** → embedding per dataset
- **SessionContext** → selezione workspace
- **LLM Router** → conversione naturali → SQL

---

## 📦 7. OUTPUT DEL PANNELLO

### File Generati nel Workspace

```
workspace/
├── datasets/...               # Dataset importati
├── assets/...                 # Asset gestiti
├── media/...                  # File multimediali
├── requirements.txt           # Dipendenze Python
├── environment.json           # Configurazione ambiente
├── db_history.json            # Storico query
├── storage_summary.json       # Riepilogo storage
└── vector_index/...           # Indici vettoriali
```

---

## ⚠️ 8. EDGE CASES

| Caso | Comportamento |
|------|---------------|
| DB non inizializzato | Bloccare query, mostrare setup wizard |
| Vector index vuoto | Preview disabilitata, suggerire ingestion |
| Dataset molto grandi | Preview limitata (primi 1000 record) |
| SQL injection | Impossibile (backend safe parser) |
| Query troppo complesse | Timeout configurabile |
| Librerie duplicate | Suggerimento cleanup automatico |
| File non supportati | Fallback raw view |

---

## 💻 9. NOTE DI IMPLEMENTAZIONE

### Ottimizzazioni Tecniche

- ✅ Utilizzare **lazy loading** delle tabelle
- ✅ Grid **virtualizzata** per dataset grandi
- ✅ Query editor con **autosave**
- ✅ Usare **Web Workers** per preview CSV/JSON pesanti

### Integrazione ChatBar

Funzionalità LLM disponibili:

```
💬 "Genera query SQL per..."
💬 "Spiega la struttura della tabella X"
💬 "Trova pattern nei dataset caricati"
💬 "Suggerisci ottimizzazioni storage"
```

---

## 🔀 10. INTEGRAZIONE CON ALTRI PANNELLI

### Con Knowledge Panel

- 📥 Ingestion dataset → grafi concettuali
- 🔗 Connessione knowledge embeddings
- 🧠 Sincronizzazione entità

### Con Logs Panel

- ❌ Errori storage riportati nei log
- 📜 Query history condivisa
- 🔍 Tracciamento operazioni

### Con Code Panel

- 📦 Lista dipendenze → patch requirements
- 🔧 Fixer auto-generato
- 🐍 Sincronizzazione virtualenv

### Con Agents Panel

- 🤖 Mostra attività KnowledgeAgent
- 📊 Mostra attività HistorianAgent
- 🔄 Workflow di ingestion

### Con Preview Panel

- 📸 Mostra snapshot stato storage
- 📊 Visualizza metriche aggregate
- ⚖️ Usage statistics

### Con Containers Panel

- 🐳 Requisiti storage per container runtime
- 💾 Mount points e volumes
- 🔧 Configurazione persistenza dati

---

<div align="center">

**💾 Storage Panel - Il Cuore dei Dati del Workspace**

*Dove i dati prendono forma, vengono organizzati e diventano conoscenza accessibile*

---

### 🚀 Prossimi Passi

> Se confermi, procedo con il **PANEL 02 — SKETCH PANEL** (già completo, ma riscritto nel template definitivo),  
> poi PANEL 03, 04, … fino al PANEL 11.

</div>