/* SPDX-License-Identifier: GPL-2.0 */
#ifndef __ASM_PCI_H
#define __ASM_PCI_H

#include <yai/types.h>
#include <yai/slab.h>
#include <yai/dma-mapping.h>

#include <yai/io.h>

#define PCIBIOS_MIN_IO		0x1000

/*
 * Set to 1 if the kernel should re-assign all PCI bus numbers
 */
#define pcibios_assign_all_busses() \
	(pci_has_flag(PCI_REASSIGN_ALL_BUS))

#define arch_can_pci_mmap_wc() 1

/* Generic PCI */
#include <yai/pci.h>

#endif  /* __ASM_PCI_H */
