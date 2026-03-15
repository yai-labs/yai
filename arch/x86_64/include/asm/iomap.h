/* SPDX-License-Identifier: GPL-2.0-or-later */
#ifndef _ASM_X86_IOMAP_H
#define _ASM_X86_IOMAP_H

/*
 * Copyright © 2008 Ingo Molnar
 */

#include <yai/fs.h>
#include <yai/mm.h>
#include <yai/uaccess.h>
#include <yai/highmem.h>
#include <yai/cacheflush.h>
#include <yai/tlbflush.h>

void __iomem *__iomap_local_pfn_prot(unsigned long pfn, pgprot_t prot);

int iomap_create_wc(resource_size_t base, unsigned long size, pgprot_t *prot);

void iomap_free(resource_size_t base, unsigned long size);

#endif /* _ASM_X86_IOMAP_H */
