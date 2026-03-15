// SPDX-License-Identifier: GPL-2.0
/*
 * IA-32 Huge TLB Page Support for Kernel.
 *
 * Copyright (C) 2002, Rohit Seth <rohit.seth@intel.com>
 */

#include <yai/init.h>
#include <yai/fs.h>
#include <yai/mm.h>
#include <yai/sched/mm.h>
#include <yai/hugetlb.h>
#include <yai/pagemap.h>
#include <yai/err.h>
#include <yai/sysctl.h>
#include <yai/compat.h>
#include <yai/mman.h>
#include <yai/tlb.h>
#include <yai/tlbflush.h>
#include <yai/elf.h>


#ifdef CONFIG_X86_64
bool __init arch_hugetlb_valid_size(unsigned long size)
{
	if (size == PMD_SIZE)
		return true;
	else if (size == PUD_SIZE && boot_cpu_has(X86_FEATURE_GBPAGES))
		return true;
	else
		return false;
}

#ifdef CONFIG_CONTIG_ALLOC
static __init int gigantic_pages_init(void)
{
	/* With compaction or CMA we can allocate gigantic pages at runtime */
	if (boot_cpu_has(X86_FEATURE_GBPAGES))
		hugetlb_add_hstate(PUD_SHIFT - PAGE_SHIFT);
	return 0;
}
arch_initcall(gigantic_pages_init);
#endif
#endif

unsigned int __init arch_hugetlb_cma_order(void)
{
	if (boot_cpu_has(X86_FEATURE_GBPAGES))
		return PUD_SHIFT - PAGE_SHIFT;

	return 0;
}
