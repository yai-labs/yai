#include <stdio.h>
#include <string.h>

#include "yai/abi/errors.h"
#include "yai/kernel/boot.h"
#include "yai/kernel/kernel.h"
#include "yai/kernel/state.h"

void yai_kernel_init_early(void) {
    struct yai_kernel_boot_handoff handoff;
    struct yai_kernel_boot_report report;

    memset(&handoff, 0, sizeof(handoff));
    memset(&report, 0, sizeof(report));

    handoff.magic = YAI_KERNEL_BOOT_HANDOFF_MAGIC;
    handoff.version = YAI_KERNEL_BOOT_HANDOFF_VERSION;
    handoff.mode = YAI_KERNEL_BOOT_MODE_NORMAL;
    handoff.boot_id = 1u;
    handoff.issued_at = 1u;

    (void)yai_kernel_boot_execute(&handoff, &report);
}

int yai_kernel_boot_execute(const struct yai_kernel_boot_handoff* handoff, struct yai_kernel_boot_report* out_report) {
    const struct yai_kernel_state* state;
    int rc;

    if (out_report != 0) {
        memset(out_report, 0, sizeof(*out_report));
    }

    rc = yai_kernel_preboot_checks(handoff, out_report);
    if (rc != YAI_OK) {
        return rc;
    }

    yai_kernel_bootstrap(handoff->boot_id);
    state = yai_kernel_state_get();
    if (state->lifecycle.current_state != YAI_KERNEL_STATE_READY) {
        if (out_report != 0) {
            snprintf(out_report->reason, sizeof(out_report->reason), "%s", "kernel_not_ready_after_bootstrap");
            out_report->lifecycle_ok = 0;
            out_report->final_state = state->lifecycle.current_state;
            out_report->readiness_flags = state->lifecycle.readiness_flags;
        }
        return YAI_ERR_DENIED;
    }

    if (out_report != 0) {
        out_report->boot_id = handoff->boot_id;
        out_report->handoff_ok = 1;
        out_report->preboot_ok = 1;
        out_report->layout_ok = 1;
        out_report->lifecycle_ok = 1;
        out_report->final_state = state->lifecycle.current_state;
        out_report->readiness_flags = state->lifecycle.readiness_flags;
        if (out_report->reason[0] == '\0') {
            snprintf(out_report->reason, sizeof(out_report->reason), "%s", "boot_ok");
        }
    }

    return YAI_OK;
}
