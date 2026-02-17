# YAI Data Plane v5 — Operational Runbook Series

**Branch:** `feat/data-plane-v5`  
**Dependencies:** v2/v3 (ws_id validation + path jail) + v4 (L2 engine attach) + logger MANDATORY

---

## Strategy: Phased Approach (v5.0 → v5.4)

Breaking v5 into sub-phases prevents "massive refactor in the dark." Each phase builds on validated traffic from L0/L1/L2.

**Why after v4:**
- Need real L0/L1/L2 traffic to validate
- Path jail must be battle-tested first
- Logger observability critical for storage debugging

**Non-negotiable prerequisites:**
1. ✅ Centralized ws_id validation + path jail active
2. ✅ Command → authority table closed and tested (arming/role)
3. ✅ Multi-stream logger operational (otherwise storage debug kills you)

---

## v5.0: Contracts + Filesystem Layout (Before Writing Any DB)

**Branch:** `feat/data-plane-v5.0-layout`

### Objective
Define ONCE AND FOR ALL where tenant DBs live (always using path jail).

### Filesystem Layout Decision

```
~/.yai/run/<ws_id>/
├── manifest.json          # DB presence manifest
├── authority.mdb          # LMDB (L1 Kernel)
│   ├── data.mdb
│   └── lock.mdb
├── events.duckdb          # DuckDB (L2 Engine)
├── engine/
│   ├── control.sock
│   └── engine.pid
└── logs/
    ├── kernel.log
    └── engine.log
```

### Redis Layout (L3 Mind)
```
yai:<ws_id>:stm:*          # Short-term memory keys
yai:<ws_id>:context:*      # Active context (pinned)
```
- Prefix per tenant
- Aggressive TTL for STM (10-60 minutes)
- Pinned keys for active context
- Unix socket local for latency + security

### DB Manifest

**File:** `~/.yai/run/<ws_id>/manifest.json`

```json
{
  "ws_id": "testws",
  "created_at": "2026-02-16T10:30:00Z",
  "owner_role": "operator",
  "storage": {
    "authority": {
      "type": "lmdb",
      "path": "authority.mdb",
      "version": "0.9.31",
      "created_at": "2026-02-16T10:30:05Z"
    },
    "events": {
      "type": "duckdb",
      "path": "events.duckdb",
      "version": "0.10.0",
      "created_at": "2026-02-16T10:30:06Z"
    },
    "redis_prefix": "yai:testws"
  }
}
```

### Files to create

**A) Spec:** `deps/yai-specs/storage/DATA_PLANE.md`

Document the layout, path rules, and isolation guarantees.

**B) Path helpers:** `kernel/src/core/storage_paths.c`

```c
#include "storage_paths.h"
#include <limits.h>
#include <stdio.h>
#include <string.h>

bool yai_storage_path_authority(char *out, size_t cap, const char *ws_id)
{
    const char *home = getenv("HOME");
    if (!home) return false;
    
    int n = snprintf(out, cap, "%s/.yai/run/%s/authority.mdb", home, ws_id);
    return n > 0 && (size_t)n < cap;
}

bool yai_storage_path_events(char *out, size_t cap, const char *ws_id)
{
    const char *home = getenv("HOME");
    if (!home) return false;
    
    int n = snprintf(out, cap, "%s/.yai/run/%s/events.duckdb", home, ws_id);
    return n > 0 && (size_t)n < cap;
}

bool yai_storage_path_manifest(char *out, size_t cap, const char *ws_id)
{
    const char *home = getenv("HOME");
    if (!home) return false;
    
    int n = snprintf(out, cap, "%s/.yai/run/%s/manifest.json", home, ws_id);
    return n > 0 && (size_t)n < cap;
}
```

### Acceptance v5.0
- [ ] `DATA_PLANE.md` spec committed
- [ ] Path helpers compile and test
- [ ] `ws create` generates manifest.json with storage section
- [ ] All paths validated through path jail

---

## v5.1: L1 Kernel — LMDB Authority Store (Minimal & Stable)

**Branch:** `feat/data-plane-v5.1-lmdb`

### Objective
Permissions/sessions/capability map in LMDB. NO large data.

### Use case
- Session → principal mapping
- Principal → capabilities/permissions
- Workspace → ownership metadata
- Authority decisions (fast lookups)

### Schema Design

**Key:** `ws_id:entity_type:entity_id`
- Example: `testws:session:sess_001`
- Example: `testws:principal:user_alice`

**Value:** Binary struct (versioned)

```c
struct yai_authority_record {
    uint32_t abi_version;  // ALWAYS first (0x00000001)
    uint64_t created_at;
    uint32_t permissions;  // bitmask
    uint8_t  role;         // guest/user/operator/sovereign
    char     data[240];    // flexible payload
} __attribute__((packed));
```

### LMDB Safety Rules (Critical)

**Zero-copy pitfall:**
LMDB gives you pointers valid ONLY while transaction is alive.

**Safe patterns:**

**Pattern A: Read-only txn per request (RECOMMENDED for v5.1)**
```c
MDB_txn *txn;
mdb_txn_begin(env, NULL, MDB_RDONLY, &txn);

MDB_val key, val;
// ... get value ...

struct yai_authority_record rec;
memcpy(&rec, val.mv_data, sizeof(rec));  // COPY to stack

mdb_txn_abort(txn);  // pointer invalid after this

// Now use 'rec' safely
```

**Pattern B: Pinned txn (MORE COMPLEX)**
- Keep txn alive with clear lifetime rules
- Document ownership explicitly
- Only if performance demands it

### Files to create/modify

**A) Header:** `kernel/include/storage_lmdb.h`

```c
#ifndef YAI_STORAGE_LMDB_H
#define YAI_STORAGE_LMDB_H

#include <lmdb.h>
#include <stdbool.h>
#include <stdint.h>

#define YAI_AUTHORITY_ABI_VERSION 0x00000001

struct yai_authority_record {
    uint32_t abi_version;
    uint64_t created_at;
    uint32_t permissions;
    uint8_t  role;
    char     data[240];
} __attribute__((packed));

typedef struct {
    MDB_env *env;
    MDB_dbi dbi;
    char ws_id[36];
    char db_path[512];
} yai_lmdb_ctx_t;

/* Open LMDB for workspace (creates if needed) */
bool yai_lmdb_open(yai_lmdb_ctx_t *ctx, const char *ws_id);

/* Close LMDB */
void yai_lmdb_close(yai_lmdb_ctx_t *ctx);

/* Write authority record */
bool yai_lmdb_put(yai_lmdb_ctx_t *ctx, 
                  const char *key,
                  const struct yai_authority_record *rec);

/* Read authority record (copies to out) */
bool yai_lmdb_get(yai_lmdb_ctx_t *ctx,
                  const char *key,
                  struct yai_authority_record *out);

/* Delete record */
bool yai_lmdb_del(yai_lmdb_ctx_t *ctx, const char *key);

#endif
```

**B) Implementation:** `kernel/src/storage/storage_lmdb.c`

```c
#include "storage_lmdb.h"
#include "storage_paths.h"
#include "logger.h"
#include <string.h>
#include <time.h>

bool yai_lmdb_open(yai_lmdb_ctx_t *ctx, const char *ws_id)
{
    if (!ctx || !ws_id) return false;
    
    memset(ctx, 0, sizeof(*ctx));
    strncpy(ctx->ws_id, ws_id, sizeof(ctx->ws_id) - 1);
    
    if (!yai_storage_path_authority(ctx->db_path, sizeof(ctx->db_path), ws_id)) {
        yai_log(YAI_LOG_WARN, ws_id, "LMDB path build failed");
        return false;
    }
    
    int rc = mdb_env_create(&ctx->env);
    if (rc != 0) {
        yai_log(YAI_LOG_WARN, ws_id, "LMDB env_create failed: %s", mdb_strerror(rc));
        return false;
    }
    
    mdb_env_set_mapsize(ctx->env, 10485760);  // 10MB initial
    mdb_env_set_maxdbs(ctx->env, 4);
    
    rc = mdb_env_open(ctx->env, ctx->db_path, MDB_NOSUBDIR, 0600);
    if (rc != 0) {
        yai_log(YAI_LOG_WARN, ws_id, "LMDB env_open failed: %s", mdb_strerror(rc));
        mdb_env_close(ctx->env);
        return false;
    }
    
    MDB_txn *txn;
    rc = mdb_txn_begin(ctx->env, NULL, 0, &txn);
    if (rc != 0) {
        yai_log(YAI_LOG_WARN, ws_id, "LMDB txn_begin failed: %s", mdb_strerror(rc));
        mdb_env_close(ctx->env);
        return false;
    }
    
    rc = mdb_dbi_open(txn, NULL, 0, &ctx->dbi);
    if (rc != 0) {
        yai_log(YAI_LOG_WARN, ws_id, "LMDB dbi_open failed: %s", mdb_strerror(rc));
        mdb_txn_abort(txn);
        mdb_env_close(ctx->env);
        return false;
    }
    
    mdb_txn_commit(txn);
    yai_log(YAI_LOG_INFO, ws_id, "LMDB opened: %s", ctx->db_path);
    return true;
}

void yai_lmdb_close(yai_lmdb_ctx_t *ctx)
{
    if (!ctx || !ctx->env) return;
    
    mdb_dbi_close(ctx->env, ctx->dbi);
    mdb_env_close(ctx->env);
    yai_log(YAI_LOG_INFO, ctx->ws_id, "LMDB closed");
    memset(ctx, 0, sizeof(*ctx));
}

bool yai_lmdb_put(yai_lmdb_ctx_t *ctx, 
                  const char *key,
                  const struct yai_authority_record *rec)
{
    if (!ctx || !ctx->env || !key || !rec) return false;
    
    MDB_txn *txn;
    int rc = mdb_txn_begin(ctx->env, NULL, 0, &txn);
    if (rc != 0) return false;
    
    MDB_val k = { .mv_size = strlen(key), .mv_data = (void*)key };
    MDB_val v = { .mv_size = sizeof(*rec), .mv_data = (void*)rec };
    
    rc = mdb_put(txn, ctx->dbi, &k, &v, 0);
    if (rc != 0) {
        mdb_txn_abort(txn);
        return false;
    }
    
    mdb_txn_commit(txn);
    return true;
}

bool yai_lmdb_get(yai_lmdb_ctx_t *ctx,
                  const char *key,
                  struct yai_authority_record *out)
{
    if (!ctx || !ctx->env || !key || !out) return false;
    
    MDB_txn *txn;
    int rc = mdb_txn_begin(ctx->env, NULL, MDB_RDONLY, &txn);
    if (rc != 0) return false;
    
    MDB_val k = { .mv_size = strlen(key), .mv_data = (void*)key };
    MDB_val v;
    
    rc = mdb_get(txn, ctx->dbi, &k, &v);
    if (rc != 0) {
        mdb_txn_abort(txn);
        return false;
    }
    
    // CRITICAL: Copy before txn_abort invalidates pointer
    memcpy(out, v.mv_data, sizeof(*out));
    
    mdb_txn_abort(txn);
    return true;
}

bool yai_lmdb_del(yai_lmdb_ctx_t *ctx, const char *key)
{
    if (!ctx || !ctx->env || !key) return false;
    
    MDB_txn *txn;
    int rc = mdb_txn_begin(ctx->env, NULL, 0, &txn);
    if (rc != 0) return false;
    
    MDB_val k = { .mv_size = strlen(key), .mv_data = (void*)key };
    
    rc = mdb_del(txn, ctx->dbi, &k, NULL);
    if (rc != 0 && rc != MDB_NOTFOUND) {
        mdb_txn_abort(txn);
        return false;
    }
    
    mdb_txn_commit(txn);
    return true;
}
```

### Integration in Kernel

**In `yai_session_acquire()`:**
```c
bool yai_session_acquire(yai_session_t **out, const char *ws_id)
{
    // ... existing code ...
    
    // Open LMDB for this workspace
    if (!yai_lmdb_open(&session->lmdb_ctx, ws_id)) {
        yai_log(YAI_LOG_WARN, ws_id, "Failed to open authority store");
        return false;
    }
    
    *out = session;
    return true;
}
```

### Acceptance v5.1
- [ ] LMDB opens for workspace
- [ ] Put/get/del operations work
- [ ] Values copied safely (no dangling pointers)
- [ ] DB file appears in manifest location
- [ ] Test with 1000 records (performance baseline)

---

## v5.2: L2 Engine — Event/Knowledge Store

**Branch:** `feat/data-plane-v5.2-events`

### Decision Point: DuckDB vs RocksDB

**Choose based on your use case:**

### Option A: DuckDB (RECOMMENDED for v5.2)

**When to use:**
- Analytical queries, joins, metrics
- Stress test aggregations
- Debug/inspection with SQL
- Event volumes: moderate (10K-1M events/day)

**Pattern:**
```c
// Writer thread with batch flush
void *event_writer_thread(void *arg) {
    while (running) {
        // Collect events from ring buffer
        Event batch[1000];
        size_t n = ring_buffer_drain(batch, 1000);
        
        // Batch insert
        duckdb_append_events(db, batch, n);
        
        // Flush every 1s or 1000 events
        if (should_flush()) {
            duckdb_flush(db);
        }
    }
}
```

### Option B: RocksDB

**When to use:**
- Very high frequency (100K+ events/sec)
- Append-only log patterns
- LSM tree benefits clear
- Don't need SQL queries

### My Recommendation

**Start with DuckDB** because:
1. SQL debugging is invaluable during development
2. Your event volumes likely moderate initially
3. Can always add RocksDB layer later if needed
4. DuckDB + Parquet export = great analytics pipeline

### Next Step Decision

**Tell me about your events:**
- How many events per second expected?
- What kind of events? (semantic ops, user actions, system metrics)
- Do you need to query/join them or just append/tail?
- Do you need real-time analytics or just audit trail?

Based on your answer, I'll write the complete v5.2 runbook with exact file targets and test matrix.

---

## v5.3: L3 Mind — Redis Short-Term Memory

**Branch:** `feat/data-plane-v5.3-redis`

*(Runbook will be written after v5.2 decision)*

---

## v5.4: Unified Data CLI (Always Through Kernel)

**Branch:** `feat/data-plane-v5.4-cli`

### Objective
CLI never accesses storage directly. All through Kernel governance.

### Commands
```bash
yai data stats --ws testws
yai data sessions --ws testws
yai data events tail --ws testws --limit 100
yai data authority get --ws testws --key "session:sess_001"
```

### Flow
```
CLI → Root → Kernel (authority check) → Query storage → Response
```

*(Full runbook after v5.2/v5.3)*

---

## Definition of Done (v5 Complete)

- [ ] v5.0: Layout + manifest operational
- [ ] v5.1: LMDB authority store working
- [ ] v5.2: Event store operational (DuckDB or RocksDB)
- [ ] v5.3: Redis STM integrated
- [ ] v5.4: Unified CLI commands working
- [ ] All storage ops through path jail
- [ ] All storage ops logged
- [ ] Test matrix passes for each sub-phase

---

**Ready to proceed with v5.2 decision. What are your event characteristics?**