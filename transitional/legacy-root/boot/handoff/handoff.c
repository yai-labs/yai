#include "yai/abi/errors.h"
#include "yai/kernel/boot.h"

int yai_kernel_boot_validate_handoff(const struct yai_kernel_boot_handoff* handoff) {
    if (handoff == 0) {
        return YAI_ERR_INVALID;
    }
    if (handoff->magic != YAI_KERNEL_BOOT_HANDOFF_MAGIC) {
        return YAI_ERR_DENIED;
    }
    if (handoff->version != YAI_KERNEL_BOOT_HANDOFF_VERSION) {
        return YAI_ERR_UNSUPPORTED;
    }
    if (handoff->boot_id == 0) {
        return YAI_ERR_INVALID;
    }
    if (handoff->mode > YAI_KERNEL_BOOT_MODE_DIAGNOSTIC) {
        return YAI_ERR_INVALID;
    }
    return YAI_OK;
}
