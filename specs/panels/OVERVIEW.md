FASE 1 – Hook di Workspace + scheletro del motore (entrypoint)

Obiettivo: ogni volta che crei/attivi un workspace, il runtime sa dove sta il project_root e ha un punto unico da cui far partire l’indicizzazione (anche solo skeleton).
Blocco 1.1 – Lifecycle Workspace/Session (hook)

File coinvolti:

    src/cortex/storage/session/workspace.py

    src/cortex/storage/session/manager.py

    src/cortex/storage/session/context.py

    src/cortex/storage/session/lifecycle.py (se ha hook di create/load)

Goal blocco:

    avere punti chiari dove:

        dopo create_workspace(...) → chiameremo il motore di indicizzazione

        dopo activate_workspace(...) → potremo fare re-sync/light-status

Senza ancora implementare il motore, qui prepariamo solo i call site (anche se puntano a funzioni “stub”).
Blocco 1.2 – WorkspaceIndexService (scheletro)

Nuovo file (o piccolo modulo) dedicato, ad esempio:

    src/cortex/knowledge/ingest/workspace_index_service.py
    oppure

    src/cortex/storage/indexes/workspace_index_service.py

Goal blocco:

    definire una classe tipo:

    class WorkspaceIndexService:
        def __init__(..., code_repository, knowledge_repository, ...): ...
        async def index_full(self, workspace_ctx): ...
        async def index_incremental(self, workspace_ctx, changed_paths: list[str]): ...
        async def status(self, workspace_ctx): ...

    per ora i metodi possono essere stub (log + TODO), ma il wiring è reale.

    esposizione di una factory o builder da usare dal SessionManager.





FASE 2 – Modello Storage: file, simboli, mapping verso Knowledge

Obiettivo: il motore ha un posto coerente dove salvare:

    info sui file del progetto,

    eventualmente simboli (classi, funzioni),

    link verso nodi Knowledge / KG.

Blocco 2.1 – Schema + modelli/base

File:

    src/cortex/storage/schema.py

    src/cortex/storage/models.py

    src/cortex/storage/migrations/*.sql (solo se serve toccare il DB)

Goal blocco:

    verificare se hai già tabelle per:

        code_files / simili

        code_symbols / simili

    se mancano o sono insufficienti, definire:

        tabella file (minimo):
        id, workspace_id, path, hash, language, last_indexed_at, is_deleted

        eventuale tabella simboli (anche minimale) con FK verso file.

    aggiornare schema.py e, se necessario, aggiungere una migration SQL.

Blocco 2.2 – Repositories code/knowledge

File:

    src/cortex/storage/repositories/code_repository.py

    src/cortex/storage/repositories/knowledge_repository.py

    eventualmente:

        src/cortex/storage/repositories/base.py

Goal blocco:

    esporre metodi high-level che userà il motore, per esempio:

        CodeRepository.list_files(workspace_id)

        CodeRepository.upsert_file(...)

        CodeRepository.mark_deleted(...)

        CodeRepository.list_dirty_files(...)

        eventuali metodi per simboli.

    in KnowledgeRepository, garantire:

        operazioni per creare/aggiornare entità collegate a file (o simboli).

        lookup per workspace_id + file_id / path.

Blocco 2.3 – Indexes/Sync (ponte logico)

File:

    src/cortex/storage/indexes/builder.py

    src/cortex/storage/indexes/sync.py

Goal blocco:

    definire una mini-API interna che:

        dato un workspace, ricostruisce gli indici (se servono, es. per RAG),

        supporta eventuali sync tra SQL e vector DB (già in parte c’è).

    preparare i metodi che il WorkspaceIndexService potrà usare per:

        sincronizzare file → embeddings (se/ quando serve),

        aggiornare eventuali index materializzati.

FASE 3 – Knowledge / Graph: dal codice al grafo

Obiettivo: collegare il motore a Knowledge, così che ogni file (e simbolo) possa diventare nodo / relazione nel grafo.
Blocco 3.1 – Ingest Service

File:

    src/cortex/knowledge/ingest/service.py

    src/cortex/knowledge/ingest/models.py

    src/cortex/knowledge/ingest/factory.py (se usato)

Goal blocco:

    definire una funzione o classe tipo:

    class KnowledgeIngestService:
        async def build_from_workspace(self, workspace_ctx, code_repo, knowledge_repo): ...

    logica (anche inizialmente minimale) per:

        prendere i file da CodeRepository

        creare “documenti di ingest” (da models.py)

        passare tali documenti al KnowledgeRepository / graph builder.

Blocco 3.2 – Graph Builder + Repository KG

File:

    src/cortex/knowledge/graph/builder.py

    src/cortex/knowledge/graph/models.py

    src/cortex/knowledge/repository/entities.py

    src/cortex/knowledge/repository/relationships.py

Goal blocco:

    far sì che esista un flusso chiaro:

        ingest → entities/relationships → graph builder.

    definire la rappresentazione minima di:

        nodo “File”

        nodo “Class”

        nodo “Function”

        relazioni FILE_CONTAINS_CLASS, CLASS_CONTAINS_FUNCTION, etc.

    senza ancora entrare nel rendering UI: questo è solo engine.

Blocco 3.3 – Query / Search semantico

File:

    src/cortex/knowledge/queries/search.py

    src/cortex/knowledge/queries/filters.py

    src/cortex/knowledge/queries/ranking.py

Goal blocco:

    aggiungere query di base tipo:

        “dammi tutti i file per workspace_id”

        “dammi i nodi collegati a un file”

        “dammi la subgraph di un file”

    in vista del fatto che File Explorer sidebar e Knowledge Panel useranno queste query.

FASE 4 – Agents: ProjectAgent come regista dell’indicizzazione

Obiettivo: usare gli agenti già esistenti per muovere il motore, invece di scrivere logica sparsa.
Blocco 4.1 – Workflow ProjectAgent

File:

    src/cortex/agents/domain/project_agent.py

Goal blocco:

    definire (almeno) due azioni:

        project.index_full

        project.index_incremental

    internamente queste azioni:

        chiamano WorkspaceIndexService

        orchestrano scanner + knowledge sync (nelle fasi successive)

Blocco 4.2 – Scanner / KnowledgeSync / Historian

File:

    src/cortex/agents/domain/scanner_agent.py

    src/cortex/agents/domain/knowledge_sync_agent.py

    src/cortex/agents/domain/historian_agent.py

Goal blocco:

    assicurare che:

        ScannerAgent sappia:

            leggere la struttura del project_root

            restituire o salvare nel CodeRepository la lista file/simboli.

        KnowledgeSyncAgent possa:

            invocare KnowledgeIngestService per aggiornare il grafo.

        HistorianAgent riceva info essenziali:

            snapshot per “timeline progetto” quando gira l’index.

    definire “contratti” chiari tra questi agenti (parametri, payload).

Blocco 4.3 – Orchestrator / Actions (solo engine lato backend)

File:

    src/cortex/orchestrator/orchestrator.py

    src/cortex/orchestrator/router.py

    src/cortex/api/registry.py

    src/cortex/api/spec.py

    src/cortex/actions/workspace_inspect.py

    eventualmente: src/cortex/actions/ui_panel.py

Goal blocco:

    registrare azioni di alto livello tipo:

        workspace.index.full

        workspace.index.status

        (in futuro) workspace.file.semantic_view

    collegare queste azioni a:

        ProjectAgent (per l’esecuzione)

        WorkspaceIndexService / KnowledgeIngestService indirettamente.

FASE 5 – Integrazione GUI / IDE (usando il motore)

Questa la tratteremo dopo che il motore è solido, ma è bene già sapere come la dividiamo.
Blocco 5.1 – Endpoint per IDE (VS Code)

File:

    src/cortex/ide/protocol.py

    src/cortex/ide/routes.py

Goal blocco:

    definire messaggi / comandi tipo:

        ide.workspace.index_status

        ide.workspace.file_tree

        ide.workspace.file_open (già c’è, ma allineato al modello nuovo)

Blocco 5.2 – IPC Workspace/FileExplorer in Electron

File (frontend/electron):

    cortex-gui/electron/main/ipc-workspace.js

    cortex-gui/electron/main/ipc-workspace-structure.js

    cortex-gui/electron/main/services/workspace-structure.js

    cortex-gui/electron/app/layout/FileExplorer.js

Goal blocco:

    usare le nuove API workspace.index.* e le query knowledge per:

        popolare la sidebar File Explorer

        mostrare stato indicizzazione / agent activity.

Blocco 5.3 – Knowledge Panel agganciato al motore

File (frontend/electron):

    cortex-gui/electron/app/panels/knowledge/*

        KnowledgeRoot.js

        KnowledgeGraphPanel.js

        KnowledgeTablePanel.js

        etc.

Goal blocco:

    usare le query di FASE 3.3 per:

        visualizzare grafo e tabelle coerenti con il file system indicizzato.




