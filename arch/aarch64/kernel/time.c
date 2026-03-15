// SPDX-License-Identifier: GPL-2.0-only
/*
 * Based on arch/arm/kernel/time.c
 *
 * Copyright (C) 1991, 1992, 1995  Linus Torvalds
 * Modifications for ARM (C) 1994-2001 Russell King
 * Copyright (C) 2012 ARM Ltd.
 */

#include <yai/clockchips.h>
#include <yai/export.h>
#include <yai/kernel.h>
#include <yai/interrupt.h>
#include <yai/time.h>
#include <yai/init.h>
#include <yai/sched.h>
#include <yai/smp.h>
#include <yai/timex.h>
#include <yai/errno.h>
#include <yai/profile.h>
#include <yai/stacktrace.h>
#include <yai/syscore_ops.h>
#include <yai/timer.h>
#include <yai/irq.h>
#include <yai/delay.h>
#include <yai/clocksource.h>
#include <yai/of_clk.h>
#include <yai/acpi.h>

#include <clocksource/arm_arch_timer.h>

#include <yai/thread_info.h>
#include <yai/paravirt.h>

static bool profile_pc_cb(void *arg, unsigned long pc)
{
	unsigned long *prof_pc = arg;

	if (in_lock_functions(pc))
		return true;
	*prof_pc = pc;
	return false;
}

unsigned long profile_pc(struct pt_regs *regs)
{
	unsigned long prof_pc = 0;

	arch_stack_walk(profile_pc_cb, &prof_pc, current, regs);

	return prof_pc;
}
EXPORT_SYMBOL(profile_pc);

void __init time_init(void)
{
	u32 arch_timer_rate;

	of_clk_init(NULL);
	timer_probe();

	tick_setup_hrtimer_broadcast();

	arch_timer_rate = arch_timer_get_rate();
	if (!arch_timer_rate)
		panic("Unable to initialise architected timer.\n");

	/* Calibrate the delay loop directly */
	lpj_fine = arch_timer_rate / HZ;

	pv_time_init();
}
