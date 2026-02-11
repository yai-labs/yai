#include "../include/engine_bridge.h"
#include "../include/engine_cortex.h"
#include <stdio.h>
#include <signal.h>
#include <unistd.h>
#include <string.h>
#include <stdlib.h>
#include <errno.h>

void handle_panic(int sig) {
    printf("\n[YAI-C] Signal %d: Emergency Lock.\n", sig);
    IceVault* v = yai_get_vault();
    {
        char addr_buf[32];
        snprintf(addr_buf, sizeof(addr_buf), "%p", (void*)v);
        setenv("YAI_VAULT_ADDR", addr_buf, 1);
    }
    if (v) {
        v->status = YAI_STATE_ERROR;
        v->authority_lock = true;
    }
    exit(sig);
}

typedef struct {
    IceVault* core;
    IceVault* stream;
    IceVault* brain;
    IceVault* audit;
    IceVault* cache;
    IceVault* control;
} IceVaultCluster;

static void validate_vault_integrity(IceVaultCluster *c) {
    if (!c || !c->core || !c->brain) return;
    if (c->core->energy_consumed > c->core->energy_quota) {
        printf("[SECURITY] Energy Quota Exceeded. Throttling Brain...\n");
        c->brain->authority_lock = true;
    }
}

static void process_command(IceVault* v) {
    if (!v) return;

    v->last_result = 0;
    v->response_buffer[0] = '\0';

    uint32_t cmd_class = yai_command_class_for((yai_command_id_t)v->last_command_id);
    if (cmd_class & YAI_CMD_CLASS_EXTERNAL) {
        if (v->authority_lock) {
            snprintf(v->last_error, sizeof(v->last_error), "External effect denied: authority required");
            v->last_result = 0;
            v->status = YAI_STATE_SUSPENDED;
            return;
        }
        snprintf(
            v->response_buffer,
            sizeof(v->response_buffer),
            "effect=external;class=%s;target=unspecified;irreversible=%s;authority=ok;intent=unspecified;risk=unspecified;mitigation=none",
            (cmd_class & YAI_CMD_CLASS_IRREVERSIBLE) ? "irreversible" : "external",
            (cmd_class & YAI_CMD_CLASS_IRREVERSIBLE) ? "true" : "false"
        );
    }

    v->status = YAI_STATE_RUNNING;
    switch (v->last_command_id) {
        case YAI_CMD_PING:
            snprintf(v->response_buffer, sizeof(v->response_buffer), "PONG");
            v->last_result = 1;
            v->status = YAI_STATE_READY;
            break;
        case YAI_CMD_NOOP:
            snprintf(v->response_buffer, sizeof(v->response_buffer), "OK");
            v->last_result = 1;
            v->status = YAI_STATE_READY;
            break;
        case YAI_CMD_RECONFIGURE:
            if (v->status != YAI_STATE_SUSPENDED) {
                snprintf(v->last_error, sizeof(v->last_error), "Reconfigure requires SUSPENDED state");
                v->last_result = 0;
                v->status = YAI_STATE_SUSPENDED;
                break;
            }
            v->authority_lock = false;
            v->status = YAI_STATE_HALT;
            snprintf(v->response_buffer, sizeof(v->response_buffer), "RECONFIGURED");
            v->last_result = 1;
            break;
        default:
            snprintf(v->last_error, sizeof(v->last_error), "Unknown command id: %u", v->last_command_id);
            v->last_result = 0;
            v->status = YAI_STATE_ERROR;
            break;
    }
}

static int engine_get_queue_depth(const IceVault* v) {
    if (!v) return 0;
    if (v->command_seq >= v->last_processed_seq) {
        return (int)(v->command_seq - v->last_processed_seq);
    }
    return 0;
}

static int read_env_int(const char* key, int fallback) {
    const char* raw = getenv(key);
    char* end = NULL;
    long v;
    if (!raw || raw[0] == '\0') return fallback;
    errno = 0;
    v = strtol(raw, &end, 10);
    if (errno != 0 || end == raw || *end != '\0') return fallback;
    return (int)v;
}

static float read_env_float(const char* key, float fallback) {
    const char* raw = getenv(key);
    char* end = NULL;
    float v;
    if (!raw || raw[0] == '\0') return fallback;
    errno = 0;
    v = strtof(raw, &end);
    if (errno != 0 || end == raw || *end != '\0') return fallback;
    return v;
}

static void apply_cortex_overrides(engine_cortex_config_t* cfg, int* initial_target) {
    if (!cfg) return;
    cfg->tick_ms = (uint32_t)read_env_int("YAI_ENGINE_CORTEX_TICK_MS", (int)cfg->tick_ms);
    cfg->ewma_alpha = read_env_float("YAI_ENGINE_CORTEX_EWMA_ALPHA", cfg->ewma_alpha);
    cfg->up_threshold = read_env_float("YAI_ENGINE_CORTEX_UP_THRESHOLD", cfg->up_threshold);
    cfg->down_threshold = read_env_float("YAI_ENGINE_CORTEX_DOWN_THRESHOLD", cfg->down_threshold);
    cfg->peak_delta = read_env_float("YAI_ENGINE_CORTEX_PEAK_DELTA", cfg->peak_delta);
    cfg->up_hold_ms = (uint32_t)read_env_int("YAI_ENGINE_CORTEX_UP_HOLD_MS", (int)cfg->up_hold_ms);
    cfg->down_hold_ms = (uint32_t)read_env_int("YAI_ENGINE_CORTEX_DOWN_HOLD_MS", (int)cfg->down_hold_ms);
    cfg->cooldown_up_ms = (uint32_t)read_env_int("YAI_ENGINE_CORTEX_COOLDOWN_UP_MS", (int)cfg->cooldown_up_ms);
    cfg->cooldown_down_ms = (uint32_t)read_env_int("YAI_ENGINE_CORTEX_COOLDOWN_DOWN_MS", (int)cfg->cooldown_down_ms);
    cfg->min_target = read_env_int("YAI_ENGINE_CORTEX_MIN_TARGET", cfg->min_target);
    cfg->max_target = read_env_int("YAI_ENGINE_CORTEX_MAX_TARGET", cfg->max_target);
    cfg->step_up = read_env_int("YAI_ENGINE_CORTEX_STEP_UP", cfg->step_up);
    cfg->step_down = read_env_int("YAI_ENGINE_CORTEX_STEP_DOWN", cfg->step_down);
    if (initial_target) {
        *initial_target = read_env_int("YAI_ENGINE_CORTEX_INITIAL_TARGET", *initial_target);
    }
}

static void emit_cortex_event(const char* ws, const engine_cortex_decision_t* d, int step) {
    const char* kind = d->direction > 0 ? "engine_scale_up" : "engine_scale_down";
    printf(
        "[YAI_CORTEX_EVENT] {\"type\":\"%s\",\"ws\":\"%s\",\"actor\":\"engine\",\"reason\":\"%s\",\"metrics\":{\"queue_depth\":%d,\"queue_ewma\":%.4f,\"peak_delta\":%.4f},\"recommendation\":{\"prev_target\":%d,\"new_target\":%d,\"step\":%d}}\n",
        kind,
        ws,
        d->reason,
        d->queue_depth,
        d->queue_ewma,
        d->peak_delta,
        d->prev_target,
        d->new_target,
        step
    );
    fflush(stdout);
}

int main(int argc, char *argv[]) {
    if (argc < 2) {
        fprintf(stderr, "USAGE: ./yai-engine <workspace_id>\n");
        return 1;
    }

    signal(SIGINT, handle_panic);
    signal(SIGTERM, handle_panic);

    IceVaultCluster cluster;
    cluster.core = yai_bridge_attach(argv[1], "");
    cluster.stream = yai_bridge_attach(argv[1], "stream");
    cluster.brain = yai_bridge_attach(argv[1], "brain");
    cluster.audit = yai_bridge_attach(argv[1], "audit");
    cluster.cache = yai_bridge_attach(argv[1], "cache");
    cluster.control = yai_bridge_attach(argv[1], "control");

    if (!cluster.core || !cluster.stream || !cluster.brain || !cluster.audit || !cluster.cache || !cluster.control) {
        fprintf(stderr, "FATAL: Multi-Vault RAID attach failed.\n");
        return 1;
    }

    IceVault* v = cluster.core;
    printf("[YAI-C] Engine Running for WS: %s\n", argv[1]);
    v->status = YAI_STATE_READY;
    v->last_processed_seq = 0;

    engine_cortex_config_t cortex_cfg = engine_cortex_default_config();
    int initial_target = cortex_cfg.min_target;
    apply_cortex_overrides(&cortex_cfg, &initial_target);
    if (engine_cortex_validate_config(&cortex_cfg) != 0) {
        fprintf(stderr, "FATAL: invalid engine cortex config\n");
        return 2;
    }

    engine_cortex_state_t cortex_st;
    engine_cortex_init(&cortex_st, &cortex_cfg, initial_target);

    uint32_t last_seen_seq = v->last_processed_seq;
    uint32_t cortex_elapsed_ms = 0;
    const uint32_t loop_sleep_us = 50000;
    const uint32_t loop_tick_ms = loop_sleep_us / 1000;

    while (v->status != YAI_STATE_ERROR && v->status != YAI_STATE_HALT) {
        validate_vault_integrity(&cluster);
        if (v->command_seq != last_seen_seq) {
            last_seen_seq = v->command_seq;
            process_command(v);
            v->last_processed_seq = last_seen_seq;
        }

        cortex_elapsed_ms += loop_tick_ms;
        if (cortex_elapsed_ms >= cortex_cfg.tick_ms) {
            int qd = engine_get_queue_depth(v);
            engine_cortex_decision_t d = engine_cortex_tick(&cortex_st, &cortex_cfg, qd);
            if (d.triggered) {
                int step = d.direction > 0 ? cortex_cfg.step_up : cortex_cfg.step_down;
                emit_cortex_event(argv[1], &d, step);
            }
            cortex_elapsed_ms = 0;
        }

        usleep(loop_sleep_us);
    }

    yai_bridge_detach();
    return 0;
}
