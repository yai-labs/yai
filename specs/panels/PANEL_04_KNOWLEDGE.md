# 📚 PANEL 04 — KNOWLEDGE PANEL

> **Workspace → Knowledge Base & Semantic Graph Management**

---

## 📋 0. OVERVIEW FUNZIONALE

Il **Knowledge Panel** è il pannello centrale per la gestione della conoscenza strutturata del workspace.

### Responsabilità Principali

- 🧠 Costruire e visualizzare la **knowledge base strutturata**
- 🕸️ Gestire grafi semantici (entità, relazioni, inferenze)
- 🔢 Generare e interrogare embedding vettoriali
- ⏱️ Mantenere timeline evolutiva del progetto
- 📸 Gestire snapshot e revisioni semantic-based
- 🔄 Sincronizzare knowledge con sistemi esterni (Obsidian)
- 🔍 Analizzare e dedurre pattern automaticamente
- 🐛 Debuggare il sistema RAG e knowledge

### Posizionamento nel Workflow

Il pannello funge da **"cervello semantico"** del workspace, integrando:

- **Knowledge Agent** → costruisce e interroga la knowledge base
- **Graph Engine** → genera ed evolve il knowledge graph
- **RAG Pipeline** → gestisce embeddings e vector search
- **Historian Agent** → costruisce timeline e snapshot
- **Knowledge Sync Agent** → esporta verso sistemi esterni
- **Project Agent** → fornisce seed knowledge iniziale

### Importanza Strategica

È l'unico pannello che integra:
- ✅ L'intero asse semantico del workspace
- ✅ La storia evolutiva del progetto
- ✅ Il sistema di retrieval augmented generation (RAG)
- ✅ L'export della conoscenza verso tool esterni

> 💡 **Il Knowledge Panel è dove i dati diventano conoscenza strutturata e navigabile.**

---

## 🖼️ 1. LAYOUT STRUTTURALE (UI)

### 1.1 Layout Globale Ereditato dal Workspace

Elementi indipendenti dal pannello:

| Elemento | Posizione | Descrizione |
|----------|-----------|-------------|
| **Chat LLM** | Right (persistente) | Query semantiche, RAG queries |
| **File Explorer** | Left | File tree del progetto |
| **TopBar** | Top | Status knowledge, sync, refresh graph |
| **Bottom Downbar** | Bottom | Pannelli 1..11 |
| **Bottom Terminal Toggle** | Bottom (sopra downbar) | Terminal / debug / output / ports |

---

### 1.2 Layout Interno del Knowledge Panel

```
 ┌─────────────────────────────────────────────────────────────────────────┐
 │ TOPBAR: Knowledge Status | Graph Nodes: 450 | Embeddings: 1.2k | Sync   │
 ├────────┬────────────────────────────────────────────────────┬───────────┤
 │        │                                                    │           │
 │  FILE  │  KNOWLEDGE STORAGE VIEW (panel-local sidebar)      │           │
 │        │   - Knowledge Base                                 │           │
 │  EXP   │      • Entities                                    │           │
 │  LOR   │      • Relationships                               │           │
 │  ER    │      • Documents                                   │           │
 │        │      • Chunks                                      │   CHAT    │
 │  (VS   │   - Embeddings                                     │   LLM     │
 │  Code  │      • Vector Index                                │           │
 │  like) │      • Models                                      │  (multi   │
 │        │      • Refresh / Dedup                             │   chat)   │
 │        │   - RAG Sessions                                   │           │
 │        │      • Query History                               │  (persi   │
 │        │      • Results                                     │   stent)  │
 │        │      • Debug Traces                                │           │
 │        │   - Snapshots                                      │  "Ask     │
 │        │      • Snapshot List                               │   Graph   │
 │        │      • Restore Preview                             │   Ask     │
 │        │      • Diff Preview                                │   RAG     │
 │        ├────────────────────────────────────────────────────┤   Query"  │
 │        │                                                    │           │
 │        │  MAIN AREA (tab-based)                             │           │
 │        │                                                    │           │
 │        │  [A] KNOWLEDGE GRAPH CANVAS (Obsidian-style)       │           │
 │        │  - Nodes: entities/chunks/documents/concepts       │           │
 │        │  - Edges: relations/similarity/inference           │           │
 │        │  - Pan/Zoom/Drag controls                          │           │
 │        │  - Filters: type/source/time/cluster               │           │
 │        │  - Overlays: heatmap/paths/influence/events        │           │
 │        │                                                    │           │
 │        │  [B] RAG EXPLORER                                  │           │
 │        │  - Embedding list & chunk viewer                   │           │
 │        │  - Vector search & top-K explorer                  │           │
 │        │  - Query execution & debugging                     │           │
 │        │  - Cluster visualization (PCA/UMAP)                │           │
 │        │                                                    │           │
 │        │  [C] KNOWLEDGE TABLE                               │           │
 │        │  - Entities / Documents / Chunks                   │           │
 │        │  - Relationships / Events / Inferences             │           │
 │        │  - Filters, search, graph preview                  │           │
 │        │                                                    │           │
 │        │  [D] TIMELINE EDITOR (DaVinci Resolve style)       │           │
 │        │  - Snapshot blocks & event markers                 │           │
 │        │  - Temporal scrubber & playback                    │           │
 │        │  - Diff viewer (knowledge/graph)                   │           │
 │        │  - Export timeline to MD                           │           │
 │        │                                                    │           │
 │        │  [E] KNOWLEDGE CONSOLE                             │           │
 │        │  - DSL commands: graph.find, rag.similar, etc.     │           │
 │        │  - Direct agent interaction                        │           │
 │        │                                                    │           │
 ├────────┴────────────────────────────────────────────────────┴───────────┤
 │ TERMINAL TOGGLE: [Terminal] [Debug Console] [Output] [Ports]            │
 ├─────────────────────────────────────────────────────────────────────────┤
 │ DOWNBAR: [①Storage][②Sketch][③Agents][④Knowledge]...[⑪Settings]       │
 └─────────────────────────────────────────────────────────────────────────┘
```

---

## 🧩 2. COMPONENTI PRINCIPALI DEL PANNELLO

### 2.1 KNOWLEDGE GRAPH CANVAS (Obsidian-style)

Visualizzazione interattiva del grafo semantico.

#### Elementi del Grafo

**Nodi:**
- Entità (concepts, modules, classes)
- Chunk di testo
- Documenti
- File sorgente
- Eventi
- Refactor points

**Edges:**
- Relazioni semantiche
- Inferenze
- Similarità embeddings
- Tracce temporali

#### Funzionalità Interattive

| Funzione | Descrizione |
|----------|-------------|
| **Pan/Zoom** | Navigazione fluida del grafo |
| **Drag nodes** | Riorganizzazione manuale |
| **Double-click** | Apre detail panel con metadati |
| **Context menu** | Azioni rapide su nodi/edge |

#### Filtri Disponibili

- 🏷️ **Entity type** (module/class/function/concept)
- 📁 **Source** (file/planner/logs/code)
- ⏰ **Time-slice** (snapshot temporale)
- 🎯 **Cluster** (embedding clusters)

#### Overlays

- 🌡️ **Heatmap** embeddings
- 🛤️ **Path highlight** (shortest path)
- 📊 **Influence graph** (impact analysis)
- 📍 **Event markers** (Historian)

#### Backend Coinvolti

```
graph.build
graph.search
knowledge.entity.get
knowledge.relationship.list
historian.timeline.events
```

---

### 2.2 RAG EXPLORER

UI completa per il sistema Retrieval Augmented Generation.

#### Sezioni Principali

**A. Embedding Management**
- Lista embedding generati
- Chunk viewer con preview
- Status modelli embedding
- Operazioni: refresh, dedup, reindex

**B. Vector Search**
- Query testuale → vector search
- Top-K results explorer
- Similarity scores visualization
- Context scoring

**C. RAG Debugging**
- Chunk selection pipeline
- Prompt building steps
- Context assembly
- LLM response analysis

**D. Cluster Visualization**
- PCA/UMAP projection
- Embedding clusters
- Semantic grouping
- Outlier detection

#### Funzionalità Avanzate

```
💬 Query: "authentication flow"
  ↓
📊 Vector search results (top-10)
  ↓
🎯 Highlight nodes in Graph Canvas
  ↓
📈 Show similarity overlay
```

#### Backend Coinvolti

```
rag.query
rag.vector.search
rag.sessions.list
embedding.generate
embedding.status
```

---

### 2.3 KNOWLEDGE TABLE

Vista tabellare strutturata della knowledge base.

#### Tabelle Disponibili

| Tabella | Contenuto |
|---------|-----------|
| **Entities** | Tutte le entità della knowledge base |
| **Documents** | Documenti ingestiti |
| **Chunks** | Chunk di testo per RAG |
| **Relationships** | Relazioni tra entità |
| **Events** | Eventi timeline (Historian) |
| **Inferences** | Inferenze automatiche |

#### Funzionalità per Tabella

- 🔍 **Filtri** multipli
- 🔎 **Ricerca** full-text
- 📊 **Open in graph** → highlight nel canvas
- 📄 **Open raw JSON** → vista metadati completi
- 🔄 **Sync to Obsidian** → export via KnowledgeSyncAgent
- 👁️ **Preview** contenuto inline

#### Agenti Coinvolti

- **KnowledgeAgent** → CRUD operations
- **HistorianAgent** → eventi e snapshot
- **KnowledgeSyncAgent** → export e sincronizzazione

---

### 2.4 TIMELINE EDITOR (DaVinci Resolve Style)

Timeline interattiva degli eventi del progetto.

#### Blocchi Timeline

**Snapshot Blocks:**
- 📸 Snapshot knowledge
- 🕸️ Snapshot grafo
- 🤖 Eventi agenti

**Event Markers:**
- 📥 Ingest/update documenti
- 📋 Planner → steps
- 🔍 Scanner → file changed
- 📊 Analyzer → hotspot
- ✅ Validator → tests
- 🔧 Codegen → patch & diff

#### Funzionalità Interattive

| Funzione | Descrizione |
|----------|-------------|
| **Temporal scrubber** | Navigazione temporale |
| **Snapshot diff** | Confronto knowledge/graph tra snapshot |
| **Markers** | Annotazioni temporali |
| **Playback** | Riproduzione sequenza eventi |
| **Export** | Timeline → MD (via KnowledgeSyncAgent) |

#### Backend Coinvolti

```
historian.timeline.get
historian.snapshot.diff
knowledge.sync.timeline
```

---

### 2.5 KNOWLEDGE CONSOLE

Terminale specializzato con DSL interno per operazioni avanzate.

#### Comandi Disponibili

**Graph Operations:**
```bash
graph.find("Auth → Login")
graph.paths("User", "Session")
graph.cluster("authentication")
```

**Knowledge Operations:**
```bash
knowledge.entity.update("User", {...})
knowledge.ingest("docs/security.md")
knowledge.search("password reset")
```

**RAG Operations:**
```bash
rag.similar("reset password")
rag.reindex()
rag.query("how to authenticate")
```

**Timeline Operations:**
```bash
timeline.snapshot("PreRefactor")
timeline.diff("v1", "v2")
timeline.export("refactor_history.md")
```

**Sync Operations:**
```bash
sync.obsidian("knowledge.md")
sync.timeline("project_timeline.md")
sync.analysis("hotspots.md")
```

#### Agenti Coinvolti

- KnowledgeAgent
- GraphBuilder
- RAGPipeline
- HistorianAgent
- KnowledgeSyncAgent

---

### 2.6 KNOWLEDGE STORAGE VIEW (Panel-Local Sidebar)

Organizzazione gerarchica delle risorse knowledge.

#### Struttura

```
📚 Knowledge Base
  ├── Entities (450)
  ├── Relationships (320)
  ├── Documents (85)
  └── Chunks (1,200)

🔢 Embeddings
  ├── Vector Index (FAISS)
  ├── Models (sentence-transformers)
  ├── Refresh
  └── Dedup

🔍 RAG Sessions
  ├── Query History (45)
  ├── Results Cache
  └── Debug Traces

📸 Snapshots
  ├── Snapshot List (12)
  ├── Restore Preview
  └── Diff Preview
```

---

## ⚙️ 3. RUOLI DEGLI AGENTI (Architettura)

### 3.1 KnowledgeAgent

**Il motore centrale della knowledge.**

#### Responsabilità

- ✅ Crea e aggiorna entità
- ✅ Gestisce versioni knowledge
- ✅ Analisi semantica
- ✅ Collega chunk ai documenti
- ✅ Aggiorna grafo tramite GraphBuilder
- ✅ Comunica con RAGPipeline

#### Backend Actions

```
knowledge.entity.create
knowledge.ingest.document
knowledge.search
knowledge.update
knowledge.graph.update
```

---

### 3.2 GraphBuilder / GraphEngine

**Costruttore e gestore del grafo semantico.**

#### Elementi Generati

- 🔵 Entity nodes
- 📄 Chunk nodes
- 🔗 Relationship edges
- 🎯 Similarity edges
- 💡 Inference edges

#### Backend Actions

```
graph.build
graph.infer
graph.search
graph.cluster
```

#### Usato Da

- KnowledgeAgent
- RAGPipeline
- HistorianAgent

---

### 3.3 RAGPipeline / EmbeddingEngine

**Sistema completo di retrieval augmented generation.**

#### Funzionalità

- 🔢 Generazione embeddings
- ✂️ Chunking intelligente
- 📊 Vector index management
- 🔍 Sessioni retrieval
- 📈 Reranking results
- 🧠 Context builder per LLM

#### Backend Actions

```
rag.vector.search
rag.query
rag.ingest
embedding.generate
embedding.status
```

---

### 3.4 HistorianAgent

**Costruttore della timeline evolutiva del progetto.**

#### Elementi Gestiti

- ⏱️ Timeline degli eventi
- 📸 Snapshot del grafo
- 💾 Snapshot della knowledge
- 🔍 Tracce (TraceBuilder)
- 📍 Markers del progetto
- 🔗 Links tra planner/code/logs/graph/rag

#### Backend Actions

```
timeline.push_event
timeline.snapshot
timeline.diff
```

#### UI nel Pannello

- Timeline Editor
- Event table
- Snapshot viewer

---

### 3.5 KnowledgeSyncAgent

**Agente di export e sincronizzazione esterna.**

> 💡 **È l'unico agente di "external publishing" della Knowledge.**

#### Funzionalità di Export

**A. Export → Obsidian (Markdown)**
- Summary knowledge
- Items list
- Entity list
- Chunk list
- Relationship map

**B. Export Timeline**
- `sync_timeline`
- MD navigabile
- Snapshot e markers

**C. Export Refactor Report**
- Diff changes
- Proposed code
- Reasoning

**D. Export Analysis**
- Summary AnalyzerAgent
- Reasoning
- Hotspots

**E. Export RAG Results**
- Query
- Answer
- Raw context

#### UI nel Pannello

- 🔄 "Sync to Obsidian"
- 📤 "Export Timeline"
- 💾 "Export Query Result"
- 📋 "Export Refactor Report"
- ✅ Vista stato sync (success/failure)

---

### 3.6 ProjectAgent

**Agente cross-panel che produce seed knowledge.**

> ⚠️ Non appartiene ad un singolo pannello.

#### Produce Dati Per

**Sketch Panel:**
- Steps, plan
- Analyzer, scanner
- Validator results

**Knowledge Panel:**
- Knowledge base iniziale
- Analysis metadata
- Scanner results
- Planner reasoning
- Test results
- Codegen patch
- Timeline updates

> 💡 **È il collegamento tra il concettuale (Sketch) e il semantico (Knowledge).**

---

## 🔌 4. INTERAZIONI BACKEND (WS REQUESTS)

### 4.1 Graph Operations

```
method: graph.build
method: graph.search
method: graph.infer
method: graph.cluster
```

**Payload Example:**
```json
{
  "workspace_id": "...",
  "query": "authentication",
  "filters": {"type": "module", "source": "code"}
}
```

---

### 4.2 Knowledge Operations

```
method: knowledge.entity.create
method: knowledge.ingest.document
method: knowledge.search
method: knowledge.update
```

---

### 4.3 RAG Operations

```
method: rag.query
method: rag.vector.search
method: rag.ingest
method: embedding.generate
```

---

### 4.4 Timeline Operations

```
method: timeline.push_event
method: timeline.snapshot
method: timeline.diff
method: historian.timeline.get
```

---

### 4.5 Sync Operations

```
method: knowledge.sync.obsidian
method: knowledge.sync.timeline
method: knowledge.sync.analysis
```

---

## 🔄 5. STATI DEL PANNELLO

| Stato | Descrizione |
|-------|-------------|
| `empty` | Knowledge base vuota |
| `loading` | Caricamento grafo/embeddings |
| `graph_view` | Visualizzazione grafo attiva |
| `rag_query_running` | Query RAG in esecuzione |
| `timeline_playing` | Playback timeline |
| `snapshot_diffing` | Confronto snapshot |
| `syncing` | Sincronizzazione Obsidian |
| `console_active` | Console in uso |
| `error` | Errore operazione knowledge |

---

## 🔗 6. DIPENDENZE

### Agenti

- **KnowledgeAgent**
- **GraphBuilder / GraphEngine**
- **RAGPipeline / EmbeddingEngine**
- **HistorianAgent**
- **KnowledgeSyncAgent**
- **ProjectAgent** (cross-panel)

### Moduli Core

- **SessionContext**
- **EmbeddingEngine**
- **VectorStore** (FAISS/Chroma)
- **EventBus** (frontend)
- **Electron IPC**

---

## 📦 7. OUTPUT DEL PANNELLO

### File Generati nel Workspace

```
workspace/
├── knowledge/
│   ├── entities.json
│   ├── relationships.json
│   ├── documents.json
│   └── chunks.json
├── graph/
│   ├── graph.json
│   ├── snapshot_*.json
│   └── inference.json
├── embeddings/
│   ├── vector_index/
│   └── embeddings.npz
├── timeline/
│   ├── events.jsonl
│   ├── snapshots/
│   └── markers.json
├── exports/
│   ├── knowledge.md (Obsidian)
│   ├── timeline.md
│   ├── analysis.md
│   └── refactor_report.md
└── rag/
    ├── sessions.jsonl
    └── query_cache.json
```

---

## ⚠️ 8. EDGE CASES

| Caso | Comportamento |
|------|---------------|
| Knowledge base vuota | Suggerire ingestion iniziale |
| Grafo troppo grande | Lazy loading + clustering |
| Embedding generation failed | Mostra errore, suggerisce retry |
| Vector index corrotto | Ricostruzione automatica |
| Snapshot non trovato | Fallback a snapshot precedente |
| Sync Obsidian failed | Retry con exponential backoff |
| RAG query timeout | Cancellazione graceful |
| Timeline troppo lunga | Paginazione + virtual scrolling |
| Console command invalid | Help message + suggerimenti |
| Graph rendering lento | WebGL acceleration + LOD |

---

## 💻 9. NOTE DI IMPLEMENTAZIONE

### Ottimizzazioni Tecniche

- ✅ **Graph rendering:** Usare WebGL (vis.js o cytoscape.js)
- ✅ **Large datasets:** Lazy loading + virtualization
- ✅ **Embedding operations:** Web Workers
- ✅ **Timeline playback:** RequestAnimationFrame
- ✅ **Console DSL:** Parser PEG.js o custom tokenizer

### Performance Considerations

```javascript
// Graph Canvas
- Max 1000 nodes visible simultaneously
- Clustering automatico oltre questa soglia
- LOD (Level of Detail) per zoom levels

// RAG Explorer
- Chunking progressivo (streaming)
- Cache risultati vector search
- Debounce query input (300ms)

// Timeline
- Virtual scrolling per timeline lunga
- Diff computation in background thread
```

### Integrazione ChatBar

```
💬 "Mostra il grafo delle relazioni di Auth"
💬 "Trova chunk simili a 'password reset'"
💬 "Crea uno snapshot pre-refactor"
💬 "Esporta la timeline in Obsidian"
💬 "Quali entità sono cambiate nell'ultimo snapshot?"
```

---

## 🔀 10. INTEGRAZIONE CON ALTRI PANNELLI

### Con Storage Panel

- 📊 **Database knowledge** → query via Storage Panel
- 🔢 **Embeddings storage** → visualizzato in entrambi
- 💾 **Vector index management** → condiviso

### Con Sketch Panel

- 📋 **Seed knowledge** da ProjectAgent
- 🎯 **Plan entities** → ingestate in knowledge
- 🔍 **Analysis results** → visualizzati nel grafo

### Con Agents Panel

- 🤖 **Activity stream** agenti knowledge
- 📊 **Status KnowledgeAgent** → real-time
- 🔄 **Sync operations** → tracciabili

### Con Logs Panel

- ❌ **Errori knowledge** → push nei log
- 📜 **Query history** → condivisa
- 🔍 **Trace operations** → linkate

### Con Code Panel

- 🔗 **Code entities** → linkate a file sorgente
- 🔧 **Refactor operations** → timeline
- 📄 **Documentation** → ingestata come knowledge

### Con Preview Panel

- 📸 **Snapshot visualization** → stato knowledge
- 📊 **Graph preview** → mini-version
- 📈 **Metrics** → da knowledge stats

### Con Git Panel

- 🔖 **Commit markers** → timeline
- 🔀 **Branch knowledge** → separate graphs
- 📝 **Commit messages** → ingestati come context

---

<div align="center">

**📚 Knowledge Panel - Il Cervello Semantico del Workspace**

*Dove i dati diventano conoscenza strutturata, navigabile e interrogabile*

---

### 🎯 Architettura degli Agenti

```
 ┌──────────────── KNOWLEDGE PANEL ────────────────┐
 │                                                  │
 │  KnowledgeAgent ────┬──── GraphBuilder ─────────┤
 │                     │                            │
 │                     ├──── RAGPipeline ──────────┤
 │                     │                            │
 │                     └──── HistorianAgent ───────┤
 │                                                  │
 │  KnowledgeSyncAgent (export subsystem)           │
 │                                                  │
 │  ProjectAgent (seed knowledge producer)          │
 │                                                  │
 └──────────────────────────────────────────────────┘
```

</div>