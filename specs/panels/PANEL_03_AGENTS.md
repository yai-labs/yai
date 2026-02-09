# PANEL 03 — AGENTS PANEL

**Workspace → Network degli Agenti + Console Operativa**

---

## 0. OVERVIEW FUNZIONALE

L'**Agents Panel** è il pannello centrale di ispezione, lancio e configurazione degli agenti AI del workspace.

Qui l'utente può:

* **Vedere lo stato in tempo reale** degli agenti (running / idle / error)
* **Eseguire singoli agenti** o "run all"
* **Interagire con ciascun agente** tramite una mini-console contestuale dedicata
* **Ricevere log di attività live** (AgentActivityPanel)
* **Configurare la pipeline LLM** locale / cloud (AgentLLMConfig)
* **Configurare la pipeline di embedding** (AgentEmbeddingConfig)
* **Monitorare errori, progressi, eventi** e aggiornamenti runtime pilotati dal backend

È l'unico pannello che offre:

* **Vista sintetica** della rete degli agenti
* **Controllo diretto** su ogni agente
* **Diagnostica completa** in tempo reale
* **Configurazione LLM e embedding** senza lasciare il workspace
* **Integrazione diretta** con tutti gli agenti del sistema

### Agenti Integrati

* **PlannerAgent** — pianificazione task e strategia
* **AnalyzerAgent** — analisi codice e dipendenze
* **ScannerAgent** — scansione file e pattern
* **KnowledgeAgent** — gestione knowledge base
* **RefactorAgent** — refactoring automatico
* **ValidatorAgent** — validazione e testing
* **LogAgent** — gestione logging avanzato
* **HistorianAgent** — tracciamento modifiche storiche
* **GitAgent** — operazioni git automatizzate
* **CodeAgent** — generazione e patching codice
* **LLM Agent** — interfaccia LLM globale

È il pannello tecnico avanzato, destinato agli utenti che vogliono "vedere cosa succede dietro al sipario".

---

## 1. LAYOUT STRUTTURALE (UI)

### 1.1 Layout globale ereditato dal Workspace

* **Chat LLM a destra** (persistente) per inviare comandi naturali relativi agli agenti
* **TopBar** per sync workspace
* **Downbar** per navigazione tra pannelli 1..11
* **Terminal Toggle** (debug console / output)
* **Sidebar file explorer** come da workspace standard

### 1.2 Layout interno dell'Agents Panel

```
 ┌──────────────────────────────────────────────────────────────────────────┐
 │ AGENTS PANEL HEADER                                                      │
 │  [Title] [Run All Button] [Refresh Status] [Config Toggle]              │
 ├──────────────────────────────────────────────────────────────────────────┤
 │ TOP CONFIG GRID (collapsible)                                            │
 │  ┌────────────────────────────┬────────────────────────────────────┐    │
 │  │ LLM Configuration          │ Embedding Configuration            │    │
 │  │ (AgentLLMConfig)           │ (AgentEmbeddingConfig)             │    │
 │  │ - Provider selection       │ - Provider selection               │    │
 │  │ - Cloud/Local mode         │ - Cloud/Local mode                 │    │
 │  │ - Model autodiscovery      │ - Model autodiscovery              │    │
 │  │ - Server start/stop        │ - Server start/stop                │    │
 │  │ - Connection test          │ - Connection test                  │    │
 │  └────────────────────────────┴────────────────────────────────────┘    │
 ├──────────────────────────────────────────────────────────────────────────┤
 │ AGENTS STATUS SECTION                                                    │
 │  ┌────────┬────────┬────────┬────────┬────────┐                         │
 │  │ AGENT  │ AGENT  │ AGENT  │ AGENT  │ AGENT  │                         │
 │  │ CARD   │ CARD   │ CARD   │ CARD   │ CARD   │                         │
 │  │ Avatar │ Avatar │ Avatar │ Avatar │ Avatar │                         │
 │  │ Status │ Status │ Status │ Status │ Status │                         │
 │  │ [Run]  │ [Run]  │ [Run]  │ [Run]  │ [Run]  │                         │
 │  └────────┴────────┴────────┴────────┴────────┘                         │
 │  ┌────────┬────────┬────────┬────────┬────────┐                         │
 │  │ AGENT  │ AGENT  │ AGENT  │ AGENT  │ AGENT  │                         │
 │  │ CARD   │ CARD   │ CARD   │ CARD   │ CARD   │                         │
 │  └────────┴────────┴────────┴────────┴────────┘                         │
 │                                                                           │
 │  [Hover on card → Mini-Console Popover]                                  │
 ├──────────────────────────────────────────────────────────────────────────┤
 │ AGENT ACTIVITY LOG (AgentActivityPanel)                                  │
 │  ┌────────────────────────────────────────────────────────────────┐     │
 │  │ [12:34:56] PlannerAgent → Task decomposition started            │     │
 │  │ [12:35:12] AnalyzerAgent → Analyzing dependencies...            │     │
 │  │ [12:35:45] KnowledgeAgent → Indexed 47 documents                │     │
 │  │ [12:36:01] PlannerAgent → Plan completed ✓                      │     │
 │  │ [12:36:23] CodeAgent → Generated patch for main.rs              │     │
 │  │ [12:36:45] ValidatorAgent → Running tests...                    │     │
 │  │ ...                                                              │     │
 │  │ [Live streaming updates from all agents]                        │     │
 │  └────────────────────────────────────────────────────────────────┘     │
 ├──────────────────────────────────────────────────────────────────────────┤
 │ CHAT LLM (sidebar destra, persistente)                                   │
 │  "Run planner agent on current workspace"                                │
 │  "Show me the last error from ValidatorAgent"                            │
 │  "Configure local LLM with llama model"                                  │
 └──────────────────────────────────────────────────────────────────────────┘
```

### 1.3 Componenti modulari inclusi

#### 1. AgentStatusPanel
Gestisce:
* Avatar grid degli agenti
* Status indicator per ogni agente
* Aggiornamenti real-time
* Run agent singolo
* Run-all batch
* Hover console contextual

#### 2. AgentConsolePanel
Mini-shell dedicata all'agente:
* Input testuale comandi
* Output stream formatted
* Log locale agente
* Comandi built-in (`/help`, `/agents`, `/reset`)
* Event bus integration

#### 3. AgentActivityPanel
Timeline verticale:
* Log eventi cronologici
* Stream updates live
* Filter per agente
* Search in logs
* Export activity

#### 4. AgentLLMConfig
UI completa per:
* Provider LLM (OpenAI, Anthropic, Local, etc.)
* Modalità cloud/local
* Endpoint configuration
* API key management
* Auto-start server locale
* Autodiscovery modelli/binari
* Model parameters (temperature, max_tokens, top_p)
* Connection testing

#### 5. AgentEmbeddingConfig
Configurazione embedding engine:
* Provider embeddings
* Modalità local/cloud
* Autodiscovery modelli
* Server management (start/stop)
* Test embedding queries
* Network parameters
* Model selection

---

## 2. COMPONENTI PRINCIPALI DEL PANNELLO

### 2.1 AGENT STATUS GRID (Avatar Cards)

Ogni card mostra:

* **Avatar visuale** dell'agente (icona personalizzata)
* **Stato corrente** con color coding:
  * `idle` → verde (pronto)
  * `running` → blu animato (in esecuzione)
  * `error` → rosso (fallimento)
  * `unknown` → grigio (non raggiungibile)
  * `starting` → giallo (avvio in corso)
* **Descrizione ruolo** breve
* **Mini-console a comparsa** (hover/click)
* **Click sul badge** = run agent immediato
* **Progress indicator** per task lunghi

#### Agenti definiti nel grid

| Agente | Ruolo | Status Possibili |
|--------|-------|------------------|
| **planner** | Pianificazione strategica task | idle, running, error |
| **codegen** | Generazione codice | idle, running, error |
| **historian** | Tracciamento cronologia | idle, running, error |
| **knowledge** | Gestione knowledge base | idle, running, error |
| **log** | Sistema logging avanzato | idle, running, error |
| **scanner** | Scansione file e pattern | idle, running, error |
| **refactor** | Refactoring automatico | idle, running, error |
| **validator** | Validazione e testing | idle, running, error |
| **git** | Operazioni git | idle, running, error |

#### Interazioni Card

```javascript
// Click su avatar badge → run agent
onClick={() => {
  window.electronAPI.agentRun({
    agent: "planner",
    input: "",
    workspace_id: currentWorkspace
  });
}}

// Hover su card → mostra mini-console
onMouseEnter={() => setShowConsole(true)}
onMouseLeave(() => setShowConsole(false))
```

### 2.2 AGENT MINI-CONSOLE (AgentConsolePanel)

Console dedicata per ogni agente con funzionalità complete.

#### Funzionalità Base

* **Input testuale** con history (↑↓ keys)
* **Output formattato** con syntax highlighting
* **Gestione stream** via event bus
* **Produce log** per AgentActivityPanel
* **Auto-scroll** su nuovi messaggi
* **Copy/paste support**
* **Clear console** command

#### Comandi Built-in

```bash
/help              # Mostra comandi disponibili
/agents            # Lista agenti attivi
/reset             # Reset stato console
/status            # Status agente corrente
/history           # Mostra cronologia comandi
/clear             # Pulisci output
/config            # Mostra config agente
```

#### Event Bus Integration

```javascript
// Emissione comandi
Events.emit("agent:command", {
  target: "planner",
  command: "/status",
  timestamp: Date.now()
});

// Ricezione output
Events.on("console:output", (msg) => {
  appendToConsole(msg);
  scrollToBottom();
});

// Stream updates
Events.on("agent:stream", (data) => {
  if (data.agent === currentAgent) {
    updateOutput(data.chunk);
  }
});
```

#### Output Formatting

```javascript
// Diversi tipi di messaggio
{
  type: "info",     // Blu
  type: "success",  // Verde
  type: "error",    // Rosso
  type: "warning",  // Giallo
  type: "system",   // Grigio
  type: "stream"    // Neutro, scrolling
}
```

### 2.3 ACTIVITY STREAM (AgentActivityPanel)

Timeline scorrevole con log completo di tutte le attività.

#### Eventi Tracciati

```javascript
// Planner events
- planner:update    // Update progressivo
- planner:done      // Completamento task
- planner:error     // Errore pianificazione

// Analyzer events
- analyzer:update   // Analisi in corso
- analyzer:done     // Analisi completata
- analyzer:error    // Errore analisi

// Historian events
- historian:commit  // Nuovo commit tracciato
- historian:sync    // Sync cronologia

// Agent run results
- agent:started     // Agent avviato
- agent:completed   // Agent terminato
- agent:failed      // Agent fallito

// Console messages
- console:input     // Input utente
- console:output    // Output agente
```

#### Aggiornamento Stream

```javascript
// IPC listeners per streaming
window.electronAPI.onPlannerUpdate((data) => {
  appendActivity({
    timestamp: Date.now(),
    agent: "planner",
    type: "update",
    message: data.message,
    progress: data.progress
  });
});

window.electronAPI.onPlannerDone((result) => {
  appendActivity({
    timestamp: Date.now(),
    agent: "planner",
    type: "done",
    message: "Task completed",
    result: result
  });
});

window.electronAPI.onPlannerError((error) => {
  appendActivity({
    timestamp: Date.now(),
    agent: "planner",
    type: "error",
    message: error.message,
    stack: error.stack
  });
});

window.electronAPI.onAnalyzerUpdate((data) => {
  appendActivity({
    timestamp: Date.now(),
    agent: "analyzer",
    type: "update",
    message: data.message
  });
});
```

#### UI Features

* **Auto-scroll** su nuovi eventi
* **Filter by agent** (dropdown)
* **Filter by type** (checkboxes: info/error/warning)
* **Search in logs** (full-text)
* **Export logs** (JSON/CSV)
* **Clear all** button
* **Timestamp formatting** (relativo o assoluto)
* **Color coding** per tipo evento

### 2.4 LLM CONFIG (AgentLLMConfig)

UI completa per configurare il Large Language Model backend.

#### Sezioni Configurazione

##### 1. Provider Selection

```javascript
providers = [
  "openai",      // OpenAI API
  "anthropic",   // Claude API
  "local",       // Server locale
  "ollama",      // Ollama
  "llamacpp",    // llama.cpp
  "custom"       // Endpoint custom
]
```

##### 2. Modalità Operativa

* **Cloud Mode**
  * API Key input (encrypted)
  * Endpoint URL
  * Organization ID (optional)
  * Rate limiting config
  * Retry policy

* **Local Mode**
  * Binary path
  * Model path
  * Port configuration
  * Memory allocation
  * Thread count
  * GPU layers (optional)

##### 3. Model Parameters

```javascript
{
  temperature: 0.7,        // 0.0 - 2.0
  max_tokens: 2048,        // 1 - 32768
  top_p: 0.9,              // 0.0 - 1.0
  frequency_penalty: 0.0,  // -2.0 - 2.0
  presence_penalty: 0.0,   // -2.0 - 2.0
  stop_sequences: []       // Array di stringhe
}
```

##### 4. Autodiscovery

```javascript
// Scansione automatica modelli locali
window.electronAPI.llmLocalDiscover()
  .then(models => {
    // models = [{name, path, size, type}, ...]
    populateModelDropdown(models);
  });

// Scansione binari disponibili
window.electronAPI.llmLocalDiscoverBinaries()
  .then(binaries => {
    // binaries = [{name, path, version}, ...]
    populateBinaryDropdown(binaries);
  });
```

##### 5. Server Management

```javascript
// Start server locale
await window.electronAPI.llmLocalStart({
  binary: "/path/to/llama-server",
  model: "/path/to/model.gguf",
  port: 8080,
  threads: 8,
  ctx_size: 4096,
  gpu_layers: 35
});

// Stop server locale
await window.electronAPI.llmLocalStop();

// Check status
const status = await window.electronAPI.llmLocalStatus();
// status = {running: true, port: 8080, model: "..."}
```

##### 6. Connection Testing

```javascript
// Test connessione
const testResult = await window.electronAPI.llmConfigTest({
  provider: "local",
  endpoint: "http://localhost:8080/v1"
});

if (testResult.success) {
  showSuccess("Connection OK");
  displayModelInfo(testResult.modelInfo);
} else {
  showError(testResult.error);
}
```

#### Persistenza Config

```javascript
// Get current config
const config = await window.electronAPI.llmConfigGet();

// Save config
await window.electronAPI.llmConfigSet({
  provider: "local",
  mode: "local",
  model: "llama-3-8b",
  endpoint: "http://localhost:8080/v1",
  parameters: {
    temperature: 0.7,
    max_tokens: 2048
  }
});
```

### 2.5 EMBEDDING CONFIG (AgentEmbeddingConfig)

Configurazione parallela per il motore di embedding.

#### Provider Embeddings

```javascript
embeddingProviders = [
  "openai",           // text-embedding-3-*
  "sentence-transformers",  // Local
  "ollama",           // ollama embed
  "llamacpp",         // llama.cpp embeddings
  "custom"            // Endpoint custom
]
```

#### Configurazione Local Embedding

```javascript
{
  provider: "sentence-transformers",
  model: "all-MiniLM-L6-v2",
  model_path: "/path/to/model",
  dimension: 384,
  normalize: true,
  batch_size: 32,
  device: "cuda"  // cpu, cuda, mps
}
```

#### Autodiscovery Modelli

```javascript
// Scansione modelli embedding locali
const models = await window.electronAPI.embeddingLocalDiscover();
// models = [
//   {name: "all-MiniLM-L6-v2", dimension: 384, size: "80MB"},
//   {name: "paraphrase-multilingual", dimension: 768, size: "420MB"}
// ]
```

#### Server Management

```javascript
// Start embedding server
await window.electronAPI.embeddingLocalStart({
  model: "all-MiniLM-L6-v2",
  port: 8081,
  device: "cuda"
});

// Stop embedding server
await window.electronAPI.embeddingLocalStop();

// Status check
const status = await window.electronAPI.embeddingLocalStatus();
```

#### Test Embedding

```javascript
// Test embedding query
const testResult = await window.electronAPI.embeddingConfigTest({
  text: "This is a test sentence",
  provider: "local"
});

if (testResult.success) {
  console.log("Embedding dimension:", testResult.embedding.length);
  console.log("First values:", testResult.embedding.slice(0, 5));
}
```

---

## 3. PARAMETRI DI CONFIGURAZIONE DEL PANEL

### 3.1 Detection Agenti Attivi

Il pannello deve interrogare il backend per lo stato agenti.

```javascript
// Lista agenti disponibili
const agents = await window.electronAPI.agentsList();
// agents = ["planner", "analyzer", "scanner", ...]

// Status singolo agente
const status = await window.electronAPI.agentStatus({
  agent: "planner"
});
// status = {
//   agent: "planner",
//   status: "idle",
//   lastRun: timestamp,
//   runCount: 42,
//   errorCount: 1
// }

// Batch status check
const allStatus = await Promise.all(
  agents.map(agent => 
    window.electronAPI.agentStatus({agent})
  )
);
```

### 3.2 Configurazioni LLM e Embedding

Persistenza backend tramite IPC dedicati.

#### LLM Config IPC

```javascript
// Get config
llmConfigGet() → {provider, mode, model, parameters, ...}

// Set config
llmConfigSet(config) → {success: true}

// Local discovery
llmLocalDiscover() → [{model, path, size}, ...]
llmLocalDiscoverBinaries() → [{name, path, version}, ...]

// Local server management
llmLocalStart(options) → {success: true, port}
llmLocalStop() → {success: true}
llmLocalStatus() → {running, port, model}

// Testing
llmConfigTest(config) → {success, modelInfo, error}
```

#### Embedding Config IPC

```javascript
// Get config
embeddingConfigGet() → {provider, model, dimension, ...}

// Set config
embeddingConfigSet(config) → {success: true}

// Local discovery
embeddingLocalDiscover() → [{model, dimension, size}, ...]

// Server management
embeddingLocalStart(options) → {success: true, port}
embeddingLocalStop() → {success: true}
embeddingLocalStatus() → {running, port, model}

// Testing
embeddingConfigTest({text}) → {success, embedding, dimension}
```

### 3.3 Stream di Aggiornamento Agenti

Eventi real-time dal backend verso UI.

#### Event Listeners

```javascript
// Planner events
window.electronAPI.onPlannerUpdate((data) => {
  // data = {message, progress, step, total}
  updateActivityLog(data);
  updateAgentCard("planner", "running");
});

window.electronAPI.onPlannerDone((result) => {
  // result = {plan, tasks, duration}
  updateActivityLog({type: "done", result});
  updateAgentCard("planner", "idle");
});

window.electronAPI.onPlannerError((error) => {
  // error = {message, stack, code}
  updateActivityLog({type: "error", error});
  updateAgentCard("planner", "error");
  showNotification("Planner Error", error.message);
});

// Analyzer events
window.electronAPI.onAnalyzerUpdate((data) => {
  // data = {file, progress, analysis}
  updateActivityLog(data);
  updateAgentCard("analyzer", "running");
});

// Generic agent events
window.electronAPI.onAgentUpdate((data) => {
  // data = {agent, type, message, ...}
  updateActivityLog(data);
  updateAgentCard(data.agent, data.status);
});
```

### 3.4 Run-All Agenti

Esecuzione batch di tutti gli agenti in sequenza o parallelo.

```javascript
// Run all agents in sequence
async function runAllAgents() {
  const agents = await window.electronAPI.agentsList();
  
  for (const agent of agents) {
    try {
      const result = await window.electronAPI.agentRun({
        agent: agent,
        input: "",
        workspace_id: currentWorkspace
      });
      
      console.log(`${agent} completed:`, result);
    } catch (error) {
      console.error(`${agent} failed:`, error);
      // Continue with next agent
    }
  }
}

// Run all agents in parallel
async function runAllAgentsParallel() {
  const agents = await window.electronAPI.agentsList();
  
  const results = await Promise.allSettled(
    agents.map(agent =>
      window.electronAPI.agentRun({
        agent: agent,
        input: "",
        workspace_id: currentWorkspace
      })
    )
  );
  
  results.forEach((result, index) => {
    if (result.status === "fulfilled") {
      console.log(`${agents[index]} OK:`, result.value);
    } else {
      console.error(`${agents[index]} FAIL:`, result.reason);
    }
  });
}
```

---

## 4. INTERAZIONI BACKEND (WS / IPC)

### 4.1 Agent Run

Esecuzione singolo agente con input opzionale.

```javascript
// IPC call
const result = await window.electronAPI.agentRun({
  agent: "planner",
  input: "Create a REST API for user management",
  workspace_id: "workspace_123",
  options: {
    timeout: 300000,  // 5 minuti
    stream: true,     // Stream updates
    context: {}       // Context addizionale
  }
});

// Result structure
{
  success: true,
  agent: "planner",
  output: {
    plan: [...],
    tasks: [...],
    duration: 12500
  },
  timestamp: 1234567890,
  metadata: {}
}
```

### 4.2 Agent Status

Interrogazione stato corrente agente.

```javascript
// IPC call
const status = await window.electronAPI.agentStatus({
  agent: "planner"
});

// Status structure
{
  agent: "planner",
  status: "idle",        // idle, running, error, unknown
  lastRun: 1234567890,   // timestamp
  lastResult: {...},     // ultimo risultato
  runCount: 42,          // esecuzioni totali
  errorCount: 1,         // errori totali
  averageDuration: 8500, // ms medio
  currentTask: null      // task in corso se running
}
```

### 4.3 LLM Config IPC

Gestione completa configurazione LLM.

```javascript
// Get current config
const config = await window.electronAPI.llmConfigGet();
// Returns:
{
  provider: "local",
  mode: "local",
  model: "llama-3-8b",
  modelPath: "/models/llama-3-8b.gguf",
  endpoint: "http://localhost:8080/v1",
  parameters: {
    temperature: 0.7,
    max_tokens: 2048,
    top_p: 0.9
  },
  serverConfig: {
    port: 8080,
    threads: 8,
    ctx_size: 4096,
    gpu_layers: 35
  }
}

// Set new config
await window.electronAPI.llmConfigSet({
  provider: "anthropic",
  mode: "cloud",
  apiKey: "sk-ant-...",
  model: "claude-3-opus-20240229",
  parameters: {
    temperature: 0.7,
    max_tokens: 4096
  }
});

// Discover local models
const models = await window.electronAPI.llmLocalDiscover();
// Returns:
[
  {
    name: "llama-3-8b",
    path: "/models/llama-3-8b.gguf",
    size: 8500000000,
    type: "gguf",
    quantization: "Q4_K_M"
  },
  ...
]

// Discover binaries
const binaries = await window.electronAPI.llmLocalDiscoverBinaries();
// Returns:
[
  {
    name: "llama-server",
    path: "/usr/local/bin/llama-server",
    version: "b1234"
  },
  ...
]

// Start local server
const started = await window.electronAPI.llmLocalStart({
  binary: "/usr/local/bin/llama-server",
  model: "/models/llama-3-8b.gguf",
  port: 8080,
  threads: 8,
  ctx_size: 4096,
  gpu_layers: 35,
  batch_size: 512
});
// Returns: {success: true, port: 8080, pid: 12345}

// Stop local server
await window.electronAPI.llmLocalStop();
// Returns: {success: true}

// Check server status
const serverStatus = await window.electronAPI.llmLocalStatus();
// Returns:
{
  running: true,
  port: 8080,
  pid: 12345,
  model: "llama-3-8b",
  uptime: 3600,
  requests: 125
}

// Test connection
const testResult = await window.electronAPI.llmConfigTest({
  provider: "local",
  endpoint: "http://localhost:8080/v1"
});
// Returns:
{
  success: true,
  latency: 45,
  modelInfo: {
    name: "llama-3-8b",
    context_length: 8192
  }
}
```

### 4.4 Embedding Config IPC

Gestione configurazione embedding engine.

```javascript
// Get current config
const config = await window.electronAPI.embeddingConfigGet();
// Returns:
{
  provider: "sentence-transformers",
  model: "all-MiniLM-L6-v2",
  modelPath: "/models/embeddings/all-MiniLM-L6-v2",
  dimension: 384,
  normalize: true,
  device: "cuda",
  batchSize: 32,
  endpoint: "http://localhost:8081"
}

// Set new config
await window.electronAPI.embeddingConfigSet({
  provider: "openai",
  model: "text-embedding-3-small",
  apiKey: "sk-...",
  dimension: 1536
});

// Discover local models
const models = await window.electronAPI.embeddingLocalDiscover();
// Returns:
[
  {
    name: "all-MiniLM-L6-v2",
    dimension: 384,
    size: 80000000,
    multilingual: false
  },
  {
    name: "paraphrase-multilingual-MiniLM-L12-v2",
    dimension: 384,
    size: 420000000,
    multilingual: true
  }
]

// Start local server
await window.electronAPI.embeddingLocalStart({
  model: "all-MiniLM-L6-v2",
  port: 8081,
  device: "cuda",
  batch_size: 32
});
// Returns: {success: true, port: 8081}

// Stop local server
await window.electronAPI.embeddingLocalStop();
// Returns: {success: true}

// Check server status
const status = await window.electronAPI.embeddingLocalStatus();
// Returns:
{
  running: true,
  port: 8081,
  model: "all-MiniLM-L6-v2",
  device: "cuda",
  requests: 347
}

// Test embedding
const testResult = await window.electronAPI.embeddingConfigTest({
  text: "This is a test sentence for embedding",
  provider: "local"
});
// Returns:
{
  success: true,
  embedding: [0.123, -0.456, 0.789, ...],  // 384 values
  dimension: 384,
  latency: 12
}
```

### 4.5 Activity Stream IPC

Push events dal backend verso UI per activity log.

```javascript
// Setup listeners all'init del pannello
function setupActivityListeners() {
  
  // Planner events
  window.electronAPI.onPlannerUpdate((data) => {
    appendActivity({
      timestamp: Date.now(),
      agent: "planner",
      type: "update",
      icon: "📋",
      message: data.message,
      progress: data.progress,
      details: data
    });
  });

  window.electronAPI.onPlannerDone((result) => {
    appendActivity({
      timestamp: Date.now(),
      agent: "planner",


console.log("LLM config:", this.getLLMConfig());
    console.log("Embedding config:", this.getEmbeddingConfig());
  },
  
  // Force refresh
  async forceRefresh() {
    await refreshAllAgentStatus();
    await refreshActivityLog();
    console.log("Force refresh completed");
  },
  
  // Clear cache
  clearCache() {
    localStorage.removeItem("agent_status_cache");
    localStorage.removeItem("activity_log_cache");
    console.log("Cache cleared");
  },
  
  // Simulate agent event
  simulateEvent(agent, type, data) {
    Events.emit("agent:event", {
      agent: agent,
      type: type,
      ...data
    });
    console.log(`Simulated ${type} event for ${agent}`);
  }
};
```

---

## 12. SICUREZZA E PERMISSIONS

### 12.1 Agent Execution Permissions

```javascript
// Verifica permessi prima esecuzione
async function checkAgentPermissions(agent) {
  const permissions = await window.electronAPI.getAgentPermissions(agent);
  
  if (!permissions.canExecute) {
    showPermissionDenied({
      agent: agent,
      reason: permissions.reason,
      required: permissions.requiredPermissions
    });
    return false;
  }
  
  return true;
}
```

### 12.2 API Key Security

```javascript
// API key storage sicuro
async function saveAPIKey(provider, key) {
  // Encrypt before saving
  const encrypted = await window.electronAPI.encryptSecret(key);
  
  await window.electronAPI.llmConfigSet({
    provider: provider,
    apiKeyEncrypted: encrypted
  });
  
  // Never store plain key in localStorage or memory
}
```

### 12.3 Local Server Isolation

```javascript
// Server locali isolati
const serverConfig = {
  llm: {
    host: "127.0.0.1",  // Solo localhost
    port: 8080,
    allowedOrigins: ["http://localhost"]
  },
  embedding: {
    host: "127.0.0.1",
    port: 8081,
    allowedOrigins: ["http://localhost"]
  }
};
```

---

## 13. PERFORMANCE METRICS

### 13.1 Monitoring

```javascript
// Metriche performance panel
const metrics = {
  agentExecutionTimes: {},
  activityLogSize: 0,
  renderTime: 0,
  memoryUsage: 0,
  
  track(agent, duration) {
    if (!this.agentExecutionTimes[agent]) {
      this.agentExecutionTimes[agent] = [];
    }
    this.agentExecutionTimes[agent].push(duration);
  },
  
  getAverageExecutionTime(agent) {
    const times = this.agentExecutionTimes[agent] || [];
    if (times.length === 0) return 0;
    return times.reduce((a, b) => a + b, 0) / times.length;
  },
  
  report() {
    console.log("=== AGENTS PANEL METRICS ===");
    Object.keys(this.agentExecutionTimes).forEach(agent => {
      console.log(`${agent}: ${this.getAverageExecutionTime(agent)}ms avg`);
    });
  }
};
```

---

**Fine documentazione PANEL 03 — AGENTS PANEL**