// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2023 SiFive
 */

#include <yai/export.h>
#include <yai/preempt.h>

#include <yai/csr.h>
#include <yai/fpu.h>
#include <yai/processor.h>
#include <yai/switch_to.h>

void kernel_fpu_begin(void)
{
	preempt_disable();
	fstate_save(current, task_pt_regs(current));
	csr_set(CSR_SSTATUS, SR_FS);
}
EXPORT_SYMBOL_GPL(kernel_fpu_begin);

void kernel_fpu_end(void)
{
	csr_clear(CSR_SSTATUS, SR_FS);
	fstate_restore(current, task_pt_regs(current));
	preempt_enable();
}
EXPORT_SYMBOL_GPL(kernel_fpu_end);
