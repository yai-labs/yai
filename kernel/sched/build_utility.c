// SPDX-License-Identifier: GPL-2.0-only
/*
 * These are various utility functions of the scheduler,
 * built in a single compilation unit for build efficiency reasons.
 *
 * ( Incidentally, the size of the compilation unit is roughly
 *   comparable to core.c, fair.c, smp.c and policy.c, the other
 *   big compilation units. This helps balance build time, while
 *   coalescing source files to amortize header inclusion
 *   cost. )
 */
#include <yai/sched/clock.h>
#include <yai/sched/cputime.h>
#include <yai/sched/debug.h>
#include <yai/sched/isolation.h>
#include <yai/sched/loadavg.h>
#include <yai/sched/nohz.h>
#include <yai/sched/mm.h>
#include <yai/sched/rseq_api.h>
#include <yai/sched/task_stack.h>

#include <yai/cpufreq.h>
#include <yai/cpumask_api.h>
#include <yai/cpuset.h>
#include <yai/ctype.h>
#include <yai/debugfs.h>
#include <yai/energy_model.h>
#include <yai/hashtable_api.h>
#include <yai/irq.h>
#include <yai/kobject_api.h>
#include <yai/membarrier.h>
#include <yai/mempolicy.h>
#include <yai/nmi.h>
#include <yai/nospec.h>
#include <yai/proc_fs.h>
#include <yai/psi.h>
#include <yai/ptrace_api.h>
#include <yai/sched_clock.h>
#include <yai/security.h>
#include <yai/spinlock_api.h>
#include <yai/swait_api.h>
#include <yai/timex.h>
#include <yai/utsname.h>
#include <yai/wait_api.h>
#include <yai/workqueue_api.h>

#include <yai/prctl.h>
#include <yai/sched/types.h>

#include <yai/switch_to.h>

#include "sched.h"
#include "sched-pelt.h"
#include "stats.h"
#include "autogroup.h"

#include "clock.c"

#ifdef CONFIG_CGROUP_CPUACCT
# include "cpuacct.c"
#endif

#ifdef CONFIG_CPU_FREQ
# include "cpufreq.c"
#endif

#ifdef CONFIG_CPU_FREQ_GOV_SCHEDUTIL
# include "cpufreq_schedutil.c"
#endif

#include "debug.c"

#ifdef CONFIG_SCHEDSTATS
# include "stats.c"
#endif

#include "loadavg.c"
#include "completion.c"
#include "swait.c"
#include "wait_bit.c"
#include "wait.c"

#include "cpupri.c"
#include "stop_task.c"

#include "topology.c"

#ifdef CONFIG_SCHED_CORE
# include "core_sched.c"
#endif

#ifdef CONFIG_PSI
# include "psi.c"
#endif

#ifdef CONFIG_MEMBARRIER
# include "membarrier.c"
#endif

#ifdef CONFIG_CPU_ISOLATION
# include "isolation.c"
#endif

#ifdef CONFIG_SCHED_AUTOGROUP
# include "autogroup.c"
#endif
