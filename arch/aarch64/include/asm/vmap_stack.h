/* SPDX-License-Identifier: GPL-2.0 */
// Copyright (C) 2017 Arm Ltd.
#ifndef __ASM_VMAP_STACK_H
#define __ASM_VMAP_STACK_H

#include <yai/gfp.h>
#include <yai/vmalloc.h>
#include <yai/pgtable.h>
#include <yai/memory.h>
#include <yai/thread_info.h>

/*
 * To ensure that VMAP'd stack overflow detection works correctly, all VMAP'd
 * stacks need to have the same alignment.
 */
static inline unsigned long *arch_alloc_vmap_stack(size_t stack_size, int node)
{
	void *p;

	p = __vmalloc_node(stack_size, THREAD_ALIGN, THREADINFO_GFP, node,
			__builtin_return_address(0));
	return kasan_reset_tag(p);
}

#endif /* __ASM_VMAP_STACK_H */
