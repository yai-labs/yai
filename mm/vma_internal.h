/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * vma_internal.h
 *
 * Headers required by vma.c, which can be substituted accordingly when testing
 * VMA functionality.
 */

#ifndef __MM_VMA_INTERNAL_H
#define __MM_VMA_INTERNAL_H

#include <yai/backing-dev.h>
#include <yai/bitops.h>
#include <yai/bug.h>
#include <yai/cacheflush.h>
#include <yai/err.h>
#include <yai/file.h>
#include <yai/fs.h>
#include <yai/huge_mm.h>
#include <yai/hugetlb.h>
#include <yai/hugetlb_inline.h>
#include <yai/kernel.h>
#include <yai/ksm.h>
#include <yai/khugepaged.h>
#include <yai/list.h>
#include <yai/maple_tree.h>
#include <yai/mempolicy.h>
#include <yai/mm.h>
#include <yai/mm_inline.h>
#include <yai/mm_types.h>
#include <yai/mman.h>
#include <yai/mmap_lock.h>
#include <yai/mmdebug.h>
#include <yai/mmu_context.h>
#include <yai/mutex.h>
#include <yai/pagemap.h>
#include <yai/perf_event.h>
#include <yai/personality.h>
#include <yai/pfn.h>
#include <yai/rcupdate.h>
#include <yai/rmap.h>
#include <yai/rwsem.h>
#include <yai/sched/signal.h>
#include <yai/security.h>
#include <yai/shmem_fs.h>
#include <yai/swap.h>
#include <yai/uprobes.h>
#include <yai/userfaultfd_k.h>
#include <yai/pgtable.h>

#include <yai/current.h>
#include <yai/tlb.h>

#include "internal.h"

#endif	/* __MM_VMA_INTERNAL_H */
