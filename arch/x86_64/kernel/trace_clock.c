// SPDX-License-Identifier: GPL-2.0
/*
 * X86 trace clocks
 */
#include <yai/trace_clock.h>
#include <yai/barrier.h>
#include <yai/tsc.h>

/*
 * trace_clock_x86_tsc(): A clock that is just the cycle counter.
 *
 * Unlike the other clocks, this is not in nanoseconds.
 */
u64 notrace trace_clock_x86_tsc(void)
{
	return rdtsc_ordered();
}
