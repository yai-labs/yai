# MASTER OVERVIEW

Perfetto. Ti lascio la **MASTER OVERVIEW** pulita, con:

* ID
* blocco
* repo
* obiettivo
* dipendenze
* priorità

Così hai finalmente la mappa madre.

| ID    | Blocco                  | Repo      | Obiettivo sintetico                                            | Dipende da       | Priorità |
| ----- | ----------------------- | --------- | -------------------------------------------------------------- | ---------------- | -------- |
| YD-1  | Source Plane Foundation | `yai`     | Rifondare architettura canonica di `yai-daemon` e source plane | —                | Massima  |
| YD-2  | Source Plane Foundation | `yai`     | Creare binario standalone `yai-daemon` con build e skeleton    | YD-1             | Massima  |
| YD-3  | Source Plane Foundation | `yai`     | Definire records, IDs e contratti del source plane             | YD-1, YD-2       | Massima  |
| YD-4  | Source Plane Foundation | `yai`     | Owner ingest core + transport bridge v1                        | YD-3             | Massima  |
| YD-5  | Source Plane Foundation | `yai`     | Runtime locale daemon: scan, spool, retry, health              | YD-2, YD-3, YD-4 | Massima  |
| YD-6  | Source Plane Foundation | `yai`     | Query, inspect e graph extension per source plane              | YD-3, YD-4, YD-5 | Massima  |
| YD-7  | Source Plane Foundation | `yai`     | Verticalizzare `exec` come parte attiva del source plane       | YD-4             | Massima  |
| YDS-1 | Source Plane Foundation | `yai-sdk` | API slice SDK per source plane                                 | YD-3, YD-4       | Alta     |
| YDS-2 | Source Plane Foundation | `yai-sdk` | Estendere locator/transport/runtime target model               | YDS-1, YD-4      | Alta     |
| YDC-1 | Source Plane Foundation | `yai-cli` | Command family `yai source ...`                                | YDS-1            | Alta     |
| YDC-2 | Source Plane Foundation | `yai-cli` | Output/watch/inspect per source plane                          | YDC-1, YD-6      | Alta     |
| YDL-1 | Source Plane Foundation | `yai-law` | Governance slice per source attachment/provenance/trust        | YD-3             | Alta     |

| ID     | Blocco                         | Repo                       | Obiettivo sintetico                                 | Dipende da              | Priorità |
| ------ | ------------------------------ | -------------------------- | --------------------------------------------------- | ----------------------- | -------- |
| MF-A1  | Mesh Foundation                | `yai`, `sdk`, `cli`, `law` | Analisi madre per Governed Sovereign Mesh           | Source Plane Foundation | Massima  |
| MF-1.1 | Mesh Discovery Foundation      | `yai` (+ cross-repo docs)  | Architettura del discovery plane e node roles       | MF-A1                   | Massima  |
| MF-1.2 | Mesh Discovery Foundation      | `yai`, `sdk`               | Protocollo baseline di owner/peer discovery         | MF-1.1                  | Massima  |
| MF-1.3 | Mesh Discovery Foundation      | `yai`, `sdk`, `cli`        | Bootstrap bundle / owner discovery seed             | MF-1.2                  | Alta     |
| MF-1.4 | Mesh Discovery Foundation      | `yai`, `cli`               | Integrazione discovery LAN/WAN                      | MF-1.2, MF-1.3          | Alta     |
| MF-2.1 | Mesh Coordination Foundation   | `yai`                      | Workspace mesh membership model                     | MF-A1, YD-3             | Massima  |
| MF-2.2 | Mesh Coordination Foundation   | `yai`                      | Owner peer registry, coordination e scheduling core | MF-2.1                  | Massima  |
| MF-2.3 | Mesh Coordination Foundation   | `yai`                      | Peer awareness metadata plane                       | MF-2.2                  | Alta     |
| MF-2.4 | Mesh Coordination Foundation   | `yai`                      | Peer-aware query, graph e workspace summary         | MF-2.2, MF-2.3, YD-6    | Massima  |
| MF-2.5 | Mesh Coordination Foundation   | `yai`                      | Conflict, ordering e replay baseline                | MF-2.2, MF-2.4          | Massima  |
| MF-3.1 | Sovereign Authority Foundation | `yai`, `law`               | Peer enrollment, identity e trust bootstrap         | MF-1.2, MF-1.3          | Massima  |
| MF-3.2 | Sovereign Authority Foundation | `yai-law`                  | Mesh trust and provenance governance slice          | MF-3.1, YDL-1           | Alta     |
| MF-3.3 | Sovereign Authority Foundation | `yai`, `law`               | Boundary lock tra mesh e truth/authority owner-side | MF-2.1, MF-3.1          | Massima  |

| ID   | Blocco         | Repo                       | Obiettivo sintetico                            | Dipende da         | Priorità |
| ---- | -------------- | -------------------------- | ---------------------------------------------- | ------------------ | -------- |
| MT-1 | Mesh Transport | `yai`, `sdk`, `cli`, `law` | Secure overlay architecture slice              | MF-A1              | Massima  |
| MT-2 | Mesh Transport | `yai`, `sdk`, `cli`        | Owner remote peer ingress + endpoint hardening | MT-1, YD-4         | Massima  |
| MT-3 | Mesh Transport | `yai`, `cli`, `sdk`, `law` | WireGuard / secure overlay integration         | MT-1, MT-2, MF-3.1 | Massima  |

| ID   | Blocco        | Repo  | Obiettivo sintetico                            | Dipende da                     | Priorità |
| ---- | ------------- | ----- | ---------------------------------------------- | ------------------------------ | -------- |
| QW-1 | Qualification | `yai` | Command contract + LAN integration wave        | Source Plane Foundation minima | Massima  |
| QW-2 | Qualification | `yai` | Secure peering qualification wave              | QW-1, MT-2, MF-3.1             | Massima  |
| QW-3 | Qualification | `yai` | Scale simulation wave con peer containerizzati | QW-1, MF-2.2, MF-2.4, MF-2.5   | Alta     |
| QW-4 | Qualification | `yai` | Real-flow qualification wave con asset veri    | QW-1, QW-3                     | Massima  |
| QW-5 | Qualification | `yai` | WAN resilience qualification wave              | QW-2, QW-4                     | Alta     |
| QW-6 | Qualification | `yai` | Pre-pilot readiness review + evidence pack     | QW-1..QW-5                     | Massima  |

## Lettura pratica del tutto

### Blocco 1 — rendere reale il source plane

Queste sono le prime 12.
Senza questo blocco non esiste neanche la base.

**Core minimo da chiudere bene:**

* YD-1
* YD-2
* YD-3
* YD-4
* YD-5
* YD-6
* YD-7
* YDS-1
* YDC-1
* YDL-1

### Blocco 2 — rifondare tutto come mesh governata

Qui smetti di pensare solo in termini di owner + daemon e passi a:

* discovery plane
* coordination plane
* authority plane

**Core minimo da chiudere bene:**

* MF-A1
* MF-1.1
* MF-1.2
* MF-2.1
* MF-2.2
* MF-2.4
* MF-2.5
* MF-3.1
* MF-3.3

### Blocco 3 — rendere il tutto deployable fuori LAN

Questo è il blocco transport/overlay.

**Core minimo da chiudere bene:**

* MT-1
* MT-2
* MT-3

### Blocco 4 — qualificare davvero il sistema

Qui entri nelle onde:

* LAN
* peering
* scale
* realflow
* WAN resilience
* readiness

---

## Sequenza consigliata vera

Se dovessi ordinarle in modo brutale e produttivo, farei così.

### Wave A — Source plane minimo serio

1. YD-1
2. YD-2
3. YD-3
4. YD-4
5. YD-5
6. YD-7
7. YD-6
8. YDS-1
9. YDC-1
10. YDL-1

### Wave B — Mesh foundation

11. MF-A1
12. MF-1.1
13. MF-1.2
14. MF-1.3
15. MF-3.1
16. MF-2.1
17. MF-2.2
18. MF-2.3
19. MF-2.4
20. MF-2.5
21. MF-3.2
22. MF-3.3

### Wave C — Transport reale

23. MT-1
24. MT-2
25. MT-3
26. YDS-2
27. YDC-2
28. MF-1.4

### Wave D — Qualification

29. QW-1
30. QW-2
31. QW-3
32. QW-4
33. QW-5
34. QW-6

---

## Cosa serve davvero prima di poter anche solo parlare bene con Fabio

### Minimo architetturale serio

* YD-1..YD-7
* YDS-1
* YDC-1
* YDL-1
* MF-A1
* MF-1.1
* MF-1.2
* MF-2.1
* MF-2.2
* MF-3.1
* MT-1
* MT-2

### Minimo di qualification serio

* QW-1
* QW-3
* QW-4

### Se vuoi poter parlare anche di WAN/overlay senza raccontare fuffa

* MT-3
* QW-2
* QW-5

---

## Cosa puoi rimandare un po’

Le cose che puoi tenere leggermente più in là, senza rompere subito la fondazione, sono:

* MF-1.4
* MF-2.3
* MF-3.2
* YDC-2
* QW-5

Non perché siano inutili, ma perché prima devi far esistere bene il nucleo.

---

## Vista finale per blocco

### Source Plane Foundation

12 consegne

### Mesh Foundation

13 consegne

### Mesh Transport

3 consegne

### Qualification

6 consegne

### Totale

**34 consegne**

---

## Se vuoi la versione ancora più operativa

.
