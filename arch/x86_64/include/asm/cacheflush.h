/* SPDX-License-Identifier: GPL-2.0 */
#ifndef _ASM_X86_CACHEFLUSH_H
#define _ASM_X86_CACHEFLUSH_H

#include <yai/mm.h>

/* Caches aren't brain-dead on the intel. */
#include <yai/cacheflush.h>
#include <yai/special_insns.h>

void clflush_cache_range(void *addr, unsigned int size);

#endif /* _ASM_X86_CACHEFLUSH_H */
