---
id: RB-KERNEL-SOVEREIGNTY
title: Kernel Sovereignty
status: draft
owner: runtime
effective_date: 2026-02-19
revision: 1
supersedes: []
depends_on:
  - RB-ROOT-HARDENING
  - RB-ENGINE-ATTACH
adr_refs:
  - docs/design/adr/ADR-003-kernel-authority.md
  - docs/design/adr/ADR-008-connection-lifecycle.md
decisions:
  - docs/design/adr/ADR-003-kernel-authority.md
  - docs/design/adr/ADR-008-connection-lifecycle.md
related:
  adr:
    - docs/design/adr/ADR-003-kernel-authority.md
    - docs/design/adr/ADR-008-connection-lifecycle.md
  specs:
    - deps/yai-specs/specs/protocol/include/transport.h
    - deps/yai-specs/specs/protocol/include/auth.h
  test_plans:
    - docs/test-plans/hardfail.md
  tools:
    - tools/bin/yai-verify
    - tools/bin/yai-gate
tags:
  - runtime
  - kernel
---

# RB-KERNEL-SOVEREIGNTY — Kernel Sovereignty

## 1) Purpose
Harden kernel authority boundaries with deterministic session/state handling, formal verification, and production-grade observability.

## 2) Preconditions
- [ ] Root boundary controls are active.
- [ ] Baseline engine attach flow is available for realistic cross-plane testing.
- [ ] Logging and path-jail prerequisites are present.

## 3) Inputs
- Core kernel modules: session, transport, enforcement, logger
- Formal assets: TLA+/traceability artifacts
- Validation tooling: `tools/bin/yai-verify`, hardfail/test suites

## 4) Procedure
Execute staged hardening in this document (logger first, then session/path hardening, then stress/fault injection closure).

## 5) Verification
- Execute all acceptance criteria per step.
- Collect deterministic logs and traceability outputs for each phase closure.

## 6) Failure Modes
- Symptom: workspace isolation breaks under concurrency/stress.
  - Fix: block merge and rework session table + path guards before proceeding.
- Symptom: authority decisions differ across code paths.
  - Fix: centralize enforcement path and rerun deterministic vectors.

## 7) Rollback
- Revert active phase changes only and restore last green kernel baseline.
- Re-run core verify and handshake smoke checks before reopening.

## 8) References
- ADR: `docs/design/adr/ADR-003-kernel-authority.md`
- Runbooks: `docs/runbooks/root-hardening.md`, `docs/runbooks/engine-attach.md`
- Test plans: `docs/test-plans/hardfail.md`

## Traceability
- ADR refs:
  - `docs/design/adr/ADR-003-kernel-authority.md`
  - `docs/design/adr/ADR-008-connection-lifecycle.md`
- MPs (to be filled as phases ship):
  - `docs/milestone-packs/...`

## Appendix — Detailed Operational Notes (Legacy Detailed Content)

### YAI Kernel Sovereignty v5 — Operational Runbook

**Branch:** `feat/kernel-sovereignty-v1`  
**Dependencies:** v2/v3 (centralized ws_id validate) + v4 (L2 attach) recommended, but logger can be done immediately.

---

## Objective

Harden kernel sovereignty with formal verification, structured session management, and production-grade observability:
- **TLA+ spec** for workspace isolation invariant
- **Session table** with explicit FSM states per workspace
- **Multi-stream logger** (file persistence + colored stderr)
- **Path jail** and access verification

---

## Recommended Sequence (Without Getting Stuck)

**A) Immediate (even today):** Multi-stream logger + fix `env->ws_id[0]` warning (helps everywhere)  
**B) v4:** L2 engine attach (start/stop/status, socket per ws)  
**C) Post-v4:** SessionTable hardening + Path Jail + Access Verify + Stress Injection

**Why this is "senior method":**
1. First put tools in place (observability)
2. Then do feature (attach engine)
3. Then deep hardening (security & tables) with real feedback (engine per ws)

---

## Pre-flight (Always)

```bash
make clean
make
pkill -f yai-root-server || true
pkill -f yai-boot || true
pkill -f yai-engine || true
yai-boot --master
yai root ping
```

---

## STEP 0: Multi-Stream Logger (Production Observability)

### Why first
Gives you deterministic audit trail and real-time debugging for all subsequent work.

### Files to create/modify

**A) Header:** `kernel/include/logger.h`

```c
#ifndef YAI_LOGGER_H
#define YAI_LOGGER_H

#include <stdarg.h>
#include <stdbool.h>

typedef enum {
    YAI_LOG_INFO = 0,
    YAI_LOG_WARN = 1,
    YAI_LOG_SEC  = 2,
    YAI_LOG_FATAL= 3
} yai_log_level_t;

/* init with explicit logfile path (recommended) */
bool yai_logger_init(const char *log_path);

/* convenience: init root log under ~/.yai/run/root/root.log */
bool yai_logger_init_root(void);

/* close */
void yai_logger_close(void);

/* logging */
void yai_log(yai_log_level_t lvl, const char *ws_id, const char *fmt, ...);
void yai_logv(yai_log_level_t lvl, const char *ws_id, const char *fmt, va_list ap);

/* security denial helper */
void yai_log_deny(const char *ws_id, unsigned cmd_id, const char *reason);

#endif
```

**B) Implementation:** `kernel/src/core/logger.c`

```c
#define _POSIX_C_SOURCE 200809L

#include "logger.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <errno.h>
#include <sys/stat.h>
#include <unistd.h>
#include <limits.h>

/* -------- ANSI (stderr only) -------- */
#define ANSI_RESET   "\x1b[0m"
#define ANSI_BOLD    "\x1b[1m"
#define ANSI_GREEN   "\x1b[32m"
#define ANSI_YELLOW  "\x1b[33m"
#define ANSI_RED     "\x1b[31m"

static FILE *g_logf = NULL;
static int   g_use_color = 1;

/* mkdir -p helper */
static int mkdir_p(const char *path)
{
    char tmp[PATH_MAX];
    size_t len;

    if (!path || !path[0]) return -1;

    snprintf(tmp, sizeof(tmp), "%s", path);
    len = strlen(tmp);
    if (len == 0) return -1;
    if (tmp[len - 1] == '/') tmp[len - 1] = '\0';

    for (char *p = tmp + 1; *p; p++) {
        if (*p == '/') {
            *p = '\0';
            if (mkdir(tmp, 0755) != 0 && errno != EEXIST) return -1;
            *p = '/';
        }
    }
    if (mkdir(tmp, 0755) != 0 && errno != EEXIST) return -1;
    return 0;
}

static void now_iso(char out[32])
{
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    struct tm tm;
    gmtime_r(&ts.tv_sec, &tm);
    snprintf(out, 32, "%04d-%02d-%02dT%02d:%02d:%02dZ",
             tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday,
             tm.tm_hour, tm.tm_min, tm.tm_sec);
}

static const char *lvl_tag(yai_log_level_t lvl)
{
    switch (lvl) {
        case YAI_LOG_INFO:  return "INFO";
        case YAI_LOG_WARN:  return "WARN";
        case YAI_LOG_SEC:   return "SECURITY";
        case YAI_LOG_FATAL: return "FATAL";
        default:            return "INFO";
    }
}

static const char *lvl_color(yai_log_level_t lvl)
{
    switch (lvl) {
        case YAI_LOG_INFO:  return ANSI_GREEN;
        case YAI_LOG_WARN:  return ANSI_YELLOW;
        case YAI_LOG_SEC:   return ANSI_RED ANSI_BOLD;
        case YAI_LOG_FATAL: return ANSI_RED ANSI_BOLD;
        default:            return ANSI_GREEN;
    }
}

bool yai_logger_init(const char *log_path)
{
    if (!log_path || !log_path[0]) return false;

    /* disable color if env says so */
    const char *no = getenv("YAI_NO_COLOR");
    if (no && no[0]) g_use_color = 0;

    /* ensure directory exists */
    char dir[PATH_MAX];
    snprintf(dir, sizeof(dir), "%s", log_path);
    char *slash = strrchr(dir, '/');
    if (slash) {
        *slash = '\0';
        if (mkdir_p(dir) != 0) return false;
    }

    g_logf = fopen(log_path, "a");
    if (!g_logf) return false;

    /* line-buffer stderr; file buffered ok */
    setvbuf(stderr, NULL, _IOLBF, 0);

    char ts[32]; now_iso(ts);
    fprintf(g_logf, "\n=== ROOT START %s ===\n", ts);
    fflush(g_logf);
    return true;
}

bool yai_logger_init_root(void)
{
    const char *home = getenv("HOME");
    if (!home) home = "/tmp";

    char p[PATH_MAX];
    snprintf(p, sizeof(p), "%s/.yai/run/root/root.log", home);
    return yai_logger_init(p);
}

void yai_logger_close(void)
{
    if (g_logf) {
        fflush(g_logf);
        fclose(g_logf);
        g_logf = NULL;
    }
}

void yai_logv(yai_log_level_t lvl, const char *ws_id, const char *fmt, va_list ap)
{
    char ts[32]; now_iso(ts);
    const char *tag = lvl_tag(lvl);
    const char *ws  = (ws_id && ws_id[0]) ? ws_id : "";

    /* --- file (no color) --- */
    if (g_logf) {
        fprintf(g_logf, "[%s] [%s] ws='%s' ", ts, tag, ws);
        vfprintf(g_logf, fmt, ap);
        fputc('\n', g_logf);
        fflush(g_logf);
    }

    /* --- stderr (color) --- */
    if (g_use_color) {
        fprintf(stderr, "%s[%s]%s [%s] ws='%s' ",
                lvl_color(lvl), tag, ANSI_RESET, ts, ws);
    } else {
        fprintf(stderr, "[%s] [%s] ws='%s' ", ts, tag, ws);
    }

    va_list ap2;
    va_copy(ap2, ap);
    vfprintf(stderr, fmt, ap2);
    va_end(ap2);

    fputc('\n', stderr);
}

void yai_log(yai_log_level_t lvl, const char *ws_id, const char *fmt, ...)
{
    va_list ap;
    va_start(ap, fmt);
    yai_logv(lvl, ws_id, fmt, ap);
    va_end(ap);
}

void yai_log_deny(const char *ws_id, unsigned cmd_id, const char *reason)
{
    yai_log(YAI_LOG_SEC, ws_id, "DENY cmd=%u reason='%s'", cmd_id, reason ? reason : "");
}
```

### Integration (Root Server)

**In `main()` of root server:**
1. Call `yai_logger_init_root()` before `listen()`
2. Replace `printf`/`fprintf(stderr, ...)` with `yai_log(INFO/WARN/SEC, ws, ...)`

### Acceptance
- [ ] If `root.log` deleted, recreated on boot with `ROOT START ...`
- [ ] Denial events print red on stderr and persist in file
- [ ] Colors only on stderr, file stays clean

---

## STEP 1: TLA+ Workspace Isolation Spec

### Files to create

**A) Spec:** `deps/yai-specs/formal/YAI_KERNEL.tla`

```tla
------------------------------ MODULE YAI_KERNEL ------------------------------
EXTENDS Naturals, Sequences, FiniteSets, TLC

(*
  Multi-tenant kernel model:
  - Workspaces is a fixed set of workspace identifiers
  - Each workspace has an independent FSM state
  - Next step updates exactly one workspace at a time (isolation)
*)

CONSTANTS
  Workspaces,            \* e.g. {"system","dev","testws"}
  States                 \* e.g. {"DOWN","UP","LOCKED"}

ASSUME Workspaces /= {} /\ States /= {}

VARIABLES
  KState                 \* function: [Workspaces -> States]

Vars == << KState >>

Init ==
  /\ KState \in [Workspaces -> States]
  /\ \A w \in Workspaces: KState[w] = "DOWN"

(*
  Helper: update exactly one workspace's state
*)
UpdateOne(w, s) ==
  /\ w \in Workspaces
  /\ s \in States
  /\ KState' = [KState EXCEPT ![w] = s]

(*
  Example transitions per workspace (you can refine later with guards)
*)
StartWS(w) ==
  /\ KState[w] = "DOWN"
  /\ UpdateOne(w, "UP")

StopWS(w) ==
  /\ KState[w] = "UP"
  /\ UpdateOne(w, "DOWN")

LockWS(w) ==
  /\ KState[w] \in {"UP","DOWN"}
  /\ UpdateOne(w, "LOCKED")

UnlockWS(w) ==
  /\ KState[w] = "LOCKED"
  /\ UpdateOne(w, "DOWN")

(*
  Next: exactly one workspace step OR stutter
*)
Next ==
  \E w \in Workspaces:
    StartWS(w) \/ StopWS(w) \/ LockWS(w) \/ UnlockWS(w)
  \/ UNCHANGED Vars

(*
  Isolation invariant:
  Every step changes at most one workspace value.
  Equivalent: for any two distinct workspaces, you can't change both in one transition.
*)
IsolationInvariant ==
  \A w1 \in Workspaces:
    \A w2 \in Workspaces:
      w1 /= w2 =>
        ~((KState'[w1] /= KState[w1]) /\ (KState'[w2] /= KState[w2]))

TypeOK ==
  KState \in [Workspaces -> States]

Spec ==
  Init /\ [][Next]_Vars

THEOREM Spec => []TypeOK
THEOREM Spec => []IsolationInvariant

=============================================================================
```

**B) Config:** `deps/yai-specs/formal/YAI_KERNEL.cfg`

```
SPECIFICATION Spec
INVARIANT TypeOK
INVARIANT IsolationInvariant

CONSTANTS
  Workspaces = {"system","testws"}
  States = {"DOWN","UP","LOCKED"}
```

### Checklist
- [ ] TLC confirms `IsolationInvariant`
- [ ] If adding "spawn engine", "pid", "socket", add as per-workspace variables: `EnginePid[w]`, `EngineSock[w]`

---

## STEP 2: Session Table Refactor (Structured State)

### Target file
`kernel/include/yai_session.h`

```c
#ifndef YAI_SESSION_H
#define YAI_SESSION_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifndef MAX_WS_ID_LEN
#define MAX_WS_ID_LEN 36  /* matches envelope ws_id[36] */
#endif

#ifndef YAI_MAX_SESSIONS
#define YAI_MAX_SESSIONS 64
#endif

typedef enum {
    YAI_FSM_DOWN = 0,
    YAI_FSM_UP   = 1,
    YAI_FSM_LOCKED = 2
} yai_fsm_state_t;

typedef struct {
    char run_dir[512];
    char lock_file[512];
    char pid_file[512];        /* kernel pid (per ws) */
    char engine_pid_file[512]; /* engine pid (per ws) */
} yai_ws_paths_t;

typedef struct {
    char ws_id[MAX_WS_ID_LEN];
    yai_ws_paths_t paths;

    uint32_t permissions_mask;   /* future: bitmask capabilities */
    int      control_fd;         /* active control connection (optional) */
    yai_fsm_state_t fsm;

    bool     in_use;
} yai_session_t;

typedef struct {
    yai_session_t slots[YAI_MAX_SESSIONS];
} yai_session_table_t;

/* Global access (or pass pointer around) */
yai_session_table_t *yai_session_table(void);

/* Workspace id validate (ideally use the inline in deps/yai-specs/specs/protocol/transport.h) */
bool yai_ws_validate_id(const char *ws_id);

/* Acquire / create session */
bool yai_session_acquire(yai_session_t **out, const char *ws_id);

/* Destroy session */
bool yai_session_release(const char *ws_id);

/* Build paths (run_dir, lock/pid files) */
bool yai_ws_build_paths(yai_session_t *s, const char *ws_id);

/* Dispatch request (existing) */
struct yai_rpc_envelope; /* forward */
void yai_session_dispatch(int client_fd,
                          const struct yai_rpc_envelope *env,
                          const void *payload);

#endif /* YAI_SESSION_H */
```

### Pro note
In v6 you can replace array with hash map, but "steel tube" array is perfect for initial hardening (deterministic, bounded).

### Acceptance
- [ ] Session table initialized on kernel boot
- [ ] `yai_session_acquire` creates session with paths built
- [ ] FSM state tracked per workspace

---

## STEP 3: Path Jail (Security Hardening)

### Goal
Never allow path traversal outside `~/.yai/run/<ws_id>/`

### Implementation locations
- `yai_ws_build_paths()` in session.c
- Any file operation in workspace context

### Rules
1. **Canonical paths only:** Use `realpath()` to resolve
2. **Prefix check:** Verify all paths start with `~/.yai/run/<ws_id>/`
3. **No symlink escape:** Validate after resolution
4. **Fail hard:** Return error code, never create outside jail

### Example check
```c
bool yai_ws_path_is_safe(const char *ws_id, const char *path)
{
    char canonical[PATH_MAX];
    if (!realpath(path, canonical)) {
        return false;  /* doesn't exist or error */
    }
    
    char jail[PATH_MAX];
    snprintf(jail, sizeof(jail), "%s/.yai/run/%s/", getenv("HOME"), ws_id);
    
    char jail_canonical[PATH_MAX];
    if (!realpath(jail, jail_canonical)) {
        return false;
    }
    
    return strncmp(canonical, jail_canonical, strlen(jail_canonical)) == 0;
}
```

### Acceptance
- [ ] Attempts with `../../etc/passwd` fail deterministically
- [ ] Symlink escape attempts caught
- [ ] Only valid paths under workspace jail succeed

---

## STEP 4: Test Matrix v5 (Sovereignty + Security)

### Script
`tests/e2e/test_kernel_sovereignty.sh`

### Minimum test cases
1. **Isolation:**
   - Create ws1, create ws2
   - Start engine ws1
   - Verify ws2 unaffected
   - Stop engine ws1
   - Verify ws2 still unaffected

2. **Path jail:**
   - Attempt create with `../../malicious`
   - Attempt create with symlink escape
   - All fail deterministically

3. **FSM transitions:**
   - DOWN → UP → LOCKED → DOWN
   - Verify state logged correctly
   - Verify invalid transitions rejected

4. **Logger:**
   - Generate denial event
   - Verify red on stderr
   - Verify persisted in file without color codes

5. **Session table:**
   - Fill to MAX_SESSIONS
   - Attempt create MAX+1 → fail gracefully
   - Release sessions
   - Verify slots reusable

### Acceptance
- [ ] All tests pass deterministically
- [ ] TLA+ model passes with isolation invariant
- [ ] No security violations logged

---

## STEP 5: Stress Injection (Optional but Recommended)

### Goal
Verify system behavior under adversarial conditions

### Test scenarios
1. **Rapid create/destroy:** 1000 cycles
2. **Invalid input flood:** Malformed ws_ids, overflow attempts
3. **Concurrent operations:** Multiple engine starts simultaneously
4. **Resource exhaustion:** Fill session table, attempt overflow
5. **Path traversal battery:** 50 different escape attempts

### Tools
- `tools/ops/stress-v1.sh`
- Python test harness with threading

### Acceptance
- [ ] System remains stable under stress
- [ ] All violations logged and rejected
- [ ] No undefined behavior or crashes

---

## Definition of Done (v5)

- [ ] Multi-stream logger operational (file + stderr with colors)
- [ ] TLA+ spec passes isolation invariant
- [ ] Session table with FSM states per workspace
- [ ] Path jail prevents traversal
- [ ] Test matrix passes
- [ ] (Optional) Stress tests pass
- [ ] All security denials logged in red

---

## Integration Notes

### Minimal integration in Root Server
```c
int main(int argc, char **argv)
{
    // Early init
    if (!yai_logger_init_root()) {
        fprintf(stderr, "FATAL: logger init failed\n");
        return 1;
    }
    
    yai_log(YAI_LOG_INFO, "system", "Root server starting");
    
    // ... rest of initialization ...
    
    // Replace old logging
    // fprintf(stderr, "error") → yai_log(YAI_LOG_WARN, ws_id, "error")
    
    yai_logger_close();
    return 0;
}
```

### Fix warning throughout codebase
Replace all instances:
```c
// OLD (triggers warning)
if (!env->ws_id || strlen(env->ws_id) == 0)

// NEW (correct)
if (env->ws_id[0] == '\0')
```

---

## Next Steps After v5

Once sovereignty hardened:
1. Add workspace quota limits (disk, memory)
2. Implement graceful degradation on resource exhaustion
3. Add metrics collection per workspace
4. Document security model in `docs/security/SOVEREIGNTY.md`
