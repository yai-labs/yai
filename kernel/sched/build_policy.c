// SPDX-License-Identifier: GPL-2.0-only
/*
 * These are the scheduling policy related scheduler files, built
 * in a single compilation unit for build efficiency reasons.
 *
 * ( Incidentally, the size of the compilation unit is roughly
 *   comparable to core.c and fair.c, the other two big
 *   compilation units. This helps balance build time, while
 *   coalescing source files to amortize header inclusion
 *   cost. )
 *
 * core.c and fair.c are built separately.
 */

/* Headers: */
#include <yai/sched/clock.h>
#include <yai/sched/cputime.h>
#include <yai/sched/hotplug.h>
#include <yai/sched/isolation.h>
#include <yai/sched/posix-timers.h>
#include <yai/sched/rt.h>

#include <yai/cpuidle.h>
#include <yai/jiffies.h>
#include <yai/kobject.h>
#include <yai/livepatch.h>
#include <yai/pm.h>
#include <yai/psi.h>
#include <yai/rhashtable.h>
#include <yai/seq_buf.h>
#include <yai/seqlock_api.h>
#include <yai/slab.h>
#include <yai/suspend.h>
#include <yai/tsacct_kern.h>
#include <yai/vtime.h>
#include <yai/sysrq.h>
#include <yai/percpu-rwsem.h>

#include <yai/sched/types.h>

#include "sched.h"
#include "smp.h"

#include "autogroup.h"
#include "stats.h"
#include "pelt.h"

/* Source code modules: */

#include "idle.c"

#include "rt.c"
#include "cpudeadline.c"

#include "pelt.c"

#include "cputime.c"
#include "deadline.c"

#ifdef CONFIG_SCHED_CLASS_EXT
# include "ext_internal.h"
# include "ext.c"
# include "ext_idle.c"
#endif

#include "syscalls.c"
