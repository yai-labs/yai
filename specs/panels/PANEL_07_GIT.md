PANEL_07_GIT.md
GIT PANEL — Version Control, Diff, History, Branching & AI-Assisted Operations
1. SCOPO DEL PANNELLO

Il Git Panel fornisce controllo completo sul versioning del workspace, integrando:

gestione del repository Git locale (inizializzazione, commit, branch, merge, tag)

view delle modifiche correnti e staged

history, reflog, timeline commit

diff viewer integrato

supporto a patch applicate dal Code Panel

interazioni AI-driven (LLM) per:

auto-commit message

rewriting commit history (safe)

suggerimenti di branching strategy

merge conflict resolution

collegamento con Agents (GitAgent, CoderAgent, RefactorAgent)

scrittura timeline automaticamente tramite HistorianAgent

Il pannello Git è l’interfaccia unificata tra code, timeline, agents, logs e knowledge.

2. ELEMENTI UI FONDAMENTALI
2.1 Git Summary Header

Mostra:

branch corrente

remote tracking status

numero di file modificati / staged / untracked

pulsanti principali:

Commit

Push

Pull

Fetch

Switch Branch

New Branch

Open .gitignore

2.2 Changes Panel (diff view stile VSCode)

Due sezioni:

(A) Working Changes

File modificati non staged:

modified

added

deleted

renamed

untracked

Azioni:

stage file

stage selected lines (inline stage)

discard change

(B) Staged Changes

File staged pronti per il commit.

Azioni:

unstage file

unstage selected lines

Entrambe le sezioni si integrano con:

CodeDiffViewer.js

CodeTabs.js

CodeMetrics.js (per suggerire commit atomici)

2.3 Diff Viewer Integrato

Vista side-by-side:

nuova versione vs vecchia versione

highlight sintattico

line-by-line comparison

inline code explanations (LLM)

quick fix suggestions (RefactorAgent)

2.4 Commit Composer

Componenti:

textarea per commit message

pulsante AI → “Generate Commit Message”

suggerimenti automatici basati su:

semver

conventional commits

struttura patch

toggle per commit ampio / granular commit

Supporto Conventional Commit:

feat:
fix:
docs:
refactor:
test:
chore:
perf:
build:
ci:

2.5 History Viewer (Git Log)

Vista timeline:

grafo dei branch (stile GitKraken/SourceTree minimal)

commit graph con nodi colorati

metadata commit

autore

messaggio

hash

file modificati

patch diff

pulsanti:

checkout commit

create branch from commit

revert commit

cherry-pick

2.6 Branch Manager

Lista branch:

local

remote

merged / unmerged

Azioni:

switch

merge

rebase

delete

create new

AI suggerisce miglior branch strategy

2.7 Stash Manager

Funzioni:

creare stash

applicare stash

visualizzare diff stash

eliminare stash

name suggestion via LLM

2.8 Integration Console (dedicata al Git Panel)

Mini-console incorporata (diversa dal terminale globale).
Mostra:

output GitAgent

conflitti di merge

diff non risolti

stato rebase

3. EVENTI UI → BACKEND (WS REQUESTS)
git.repo.init
git.repo.status
git.repo.history
git.repo.branches
git.repo.stash.list

git.stage
git.unstage
git.discard
git.commit
git.push
git.pull
git.fetch
git.merge
git.rebase
git.branch.create
git.branch.delete
git.branch.switch
git.stash.create
git.stash.apply
git.stash.drop

git.diff.file
git.diff.commit
git.diff.stash

git.llm.commit_message
git.llm.merge_resolution
git.llm.branch_strategy


Tutti mappano verso GitAgent o Orchestrator → Git backend.

4. EVENTI BACKEND → UI
git.repo.updated
git.repo.status
git.diff.result
git.commit.applied
git.merge.conflict
git.merge.resolved
git.rebase.progress
git.repo.history
git.branch.switched
git.stash.updated
git.ai.suggestion
git.error
timeline.event.created
knowledge.item.created

5. INTEGRAZIONE CON ALTRI PANNELLI / AGENTI
5.1 Agenti
GitAgent

Agente principale, responsabile di:

commit

diff

history

merge

rebase

push/pull

stash

CoderAgent

Produce patch → automaticamente comparabili con diff del pannello.

RefactorAgent

Genera patch strutturate → staging assistito.

AnalyzerAgent

Fornisce metrics per commit intelligenti:

hotspots

complexity

style issues

HistorianAgent

Registra:

commit events

merge events

conflict resolutions

code edits timeline

KnowledgeSyncAgent

Permette esportazione Markdown:

refactor report

diff report

changelog auto-generato

5.2 Collegamenti con altri pannelli
[Code Panel] → patch → [Git Panel] → commit
[Agents Panel] → attività agenti → [Git Timeline]
[Logs Panel] → error logs → commit reasoning
[Knowledge Panel] → ingest file modificati
[Preview Panel] → mostra stato repository sintetico

6. STATO GESTITO DAL PANNELLO
Locale (frontend)

working diff

staged diff

commit message draft

selected commits

selected branch

queued operations (merge/rebase)

merge conflict state

Globale (workspace)

repository git

branch map

commit graph

stash list

metadata timeline

last patch applied

7. TASTI, AZIONI, SHORTCUTS

CTRL+SHIFT+G → apri pannello Git

CTRL+ENTER → commit

CTRL+ALT+S → stage all

CTRL+ALT+U → unstage all

CTRL+ALT+D → discard changes

CTRL+ALT+M → AI generate message

CTRL+ALT+B → new branch

F7 → merge

F8 → rebase

SHIFT+F7 → resolve conflict via LLM

8. OUTPUT GENERATI DAL PANNELLO

File e artefatti:

.git/* (repo)

patches/*.diff

changelog/*.md

timeline/git_events/*.json

knowledge/git_insights/*.md

9. CASI LIMITE / EDGE CASES

conflitti merge complessi

commit troppo grande → warn + suggest split

repo corrotto → modalità recovery

branch divergenti → suggerimenti LLM

push fallito → credenziali / remote non valido

rebase in stallo → abort / continue / skip

file binari in diff → fallback raw

changelog non generabile → fallback base

10. NOTE DI IMPLEMENTAZIONE

usare Git backend locale via Python (gitpython + comandi raw)

WebSocket deve essere stream-based per operazioni lunghe (fetch, clone)

diff viewer deve riutilizzare CodeDiffViewer.js

integrare highlight differenziale per patch large

prevedere “AI Safe Mode” per merge automatici

timeline commit integrata come nel Logs Panel

history viewer deve supportare grafo commit (HTML canvas + SVG)

commit atomicità: impedire commit se patch incomplete

FINE DOCUMENTO