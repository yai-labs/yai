// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2025 MIPS.
 */

#include <yai/vendor_extensions.h>
#include <yai/vendor_extensions/mips.h>
#include <yai/vendor_extensions/mips_hwprobe.h>
#include <yai/vendor_extensions/vendor_hwprobe.h>

#include <yai/cpumask.h>
#include <yai/types.h>

#include <uapi/asm/hwprobe.h>
#include <uapi/asm/vendor/mips.h>

void hwprobe_isa_vendor_ext_mips_0(struct riscv_hwprobe *pair,
				   const struct cpumask *cpus)
{
	VENDOR_EXTENSION_SUPPORTED(pair, cpus,
				   riscv_isa_vendor_ext_list_mips.per_hart_isa_bitmap,
				   { VENDOR_EXT_KEY(XMIPSEXECTL); });
}
