#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

#include "yai/abi/errors.h"
#include "yai/kernel/boot.h"

static int yai_kernel_path_is_dir(const char* path) {
    struct stat st;
    if (path == 0 || path[0] == '\0') {
        return 0;
    }
    if (stat(path, &st) != 0) {
        return 0;
    }
    return S_ISDIR(st.st_mode) ? 1 : 0;
}

int yai_kernel_preboot_checks(const struct yai_kernel_boot_handoff* handoff, struct yai_kernel_boot_report* out_report) {
    int rc = yai_kernel_boot_validate_handoff(handoff);

    if (out_report != 0) {
        out_report->handoff_ok = (rc == YAI_OK) ? 1 : 0;
    }
    if (rc != YAI_OK) {
        return rc;
    }

    if ((!yai_kernel_path_is_dir("boot/bootloader") && !yai_kernel_path_is_dir("../boot/bootloader")) ||
        (!yai_kernel_path_is_dir("boot/handoff") && !yai_kernel_path_is_dir("../boot/handoff")) ||
        (!yai_kernel_path_is_dir("boot/initrd") && !yai_kernel_path_is_dir("../boot/initrd")) ||
        (!yai_kernel_path_is_dir("boot/configs") && !yai_kernel_path_is_dir("../boot/configs"))) {
        if (out_report != 0) {
            snprintf(out_report->reason, sizeof(out_report->reason), "%s", "boot_layout_incomplete");
            out_report->layout_ok = 0;
            out_report->preboot_ok = 0;
        }
        return YAI_ERR_NOT_FOUND;
    }

    if (out_report != 0) {
        out_report->layout_ok = 1;
        out_report->preboot_ok = 1;
        if (geteuid() == 0) {
            snprintf(out_report->reason, sizeof(out_report->reason), "%s", "preboot_root_mode");
        }
    }

    return YAI_OK;
}
