// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Memory Encryption Support
 *
 * Copyright (C) 2019 SUSE
 *
 * Author: Joerg Roedel <jroedel@suse.de>
 */

#define pr_fmt(fmt)	"SEV: " fmt

#include <yai/percpu-defs.h>
#include <yai/cc_platform.h>
#include <yai/printk.h>
#include <yai/mm_types.h>
#include <yai/set_memory.h>
#include <yai/memblock.h>
#include <yai/kernel.h>
#include <yai/mm.h>
#include <yai/cpumask.h>
#include <yai/efi.h>
#include <yai/io.h>
#include <yai/psp-sev.h>
#include <yai/sev-guest.h>

#include <yai/init.h>
#include <yai/cpu_entry_area.h>
#include <yai/stacktrace.h>
#include <yai/sev.h>
#include <yai/insn-eval.h>
#include <yai/fpu/xcr.h>
#include <yai/processor.h>
#include <yai/realmode.h>
#include <yai/setup.h>
#include <yai/traps.h>
#include <yai/svm.h>
#include <yai/smp.h>
#include <yai/cpu.h>
#include <yai/apic.h>
#include <yai/cpuid/api.h>
#include <yai/cmdline.h>

#include "../../coco/sev/internal.h"

/* Include code shared with pre-decompression boot stage */
#include "sev-shared.c"

void
early_set_pages_state(unsigned long vaddr, unsigned long paddr,
		      unsigned long npages, const struct psc_desc *desc)
{
	unsigned long paddr_end;

	vaddr = vaddr & PAGE_MASK;

	paddr = paddr & PAGE_MASK;
	paddr_end = paddr + (npages << PAGE_SHIFT);

	while (paddr < paddr_end) {
		__page_state_change(vaddr, paddr, desc);

		vaddr += PAGE_SIZE;
		paddr += PAGE_SIZE;
	}
}

void __init early_snp_set_memory_private(unsigned long vaddr, unsigned long paddr,
					 unsigned long npages)
{
	struct psc_desc d = {
		SNP_PAGE_STATE_PRIVATE,
		rip_rel_ptr(&boot_svsm_ca_page),
		boot_svsm_caa_pa
	};

	/*
	 * This can be invoked in early boot while running identity mapped, so
	 * use an open coded check for SNP instead of using cc_platform_has().
	 * This eliminates worries about jump tables or checking boot_cpu_data
	 * in the cc_platform_has() function.
	 */
	if (!(sev_status & MSR_AMD64_SEV_SNP_ENABLED))
		return;

	 /*
	  * Ask the hypervisor to mark the memory pages as private in the RMP
	  * table.
	  */
	early_set_pages_state(vaddr, paddr, npages, &d);
}

void __init early_snp_set_memory_shared(unsigned long vaddr, unsigned long paddr,
					unsigned long npages)
{
	struct psc_desc d = {
		SNP_PAGE_STATE_SHARED,
		rip_rel_ptr(&boot_svsm_ca_page),
		boot_svsm_caa_pa
	};

	/*
	 * This can be invoked in early boot while running identity mapped, so
	 * use an open coded check for SNP instead of using cc_platform_has().
	 * This eliminates worries about jump tables or checking boot_cpu_data
	 * in the cc_platform_has() function.
	 */
	if (!(sev_status & MSR_AMD64_SEV_SNP_ENABLED))
		return;

	 /* Ask hypervisor to mark the memory pages shared in the RMP table. */
	early_set_pages_state(vaddr, paddr, npages, &d);
}

/*
 * Initial set up of SNP relies on information provided by the
 * Confidential Computing blob, which can be passed to the kernel
 * in the following ways, depending on how it is booted:
 *
 * - when booted via the boot/decompress kernel:
 *   - via boot_params
 *
 * - when booted directly by firmware/bootloader (e.g. CONFIG_PVH):
 *   - via a setup_data entry, as defined by the Linux Boot Protocol
 *
 * Scan for the blob in that order.
 */
static struct cc_blob_sev_info *__init find_cc_blob(struct boot_params *bp)
{
	struct cc_blob_sev_info *cc_info;

	/* Boot kernel would have passed the CC blob via boot_params. */
	if (bp->cc_blob_address) {
		cc_info = (struct cc_blob_sev_info *)(unsigned long)bp->cc_blob_address;
		goto found_cc_info;
	}

	/*
	 * If kernel was booted directly, without the use of the
	 * boot/decompression kernel, the CC blob may have been passed via
	 * setup_data instead.
	 */
	cc_info = find_cc_blob_setup_data(bp);
	if (!cc_info)
		return NULL;

found_cc_info:
	if (cc_info->magic != CC_BLOB_SEV_HDR_MAGIC)
		sev_es_terminate(SEV_TERM_SET_GEN, GHCB_SNP_UNSUPPORTED);

	return cc_info;
}

static void __init svsm_setup(struct cc_blob_sev_info *cc_info)
{
	struct snp_secrets_page *secrets = (void *)cc_info->secrets_phys;
	struct svsm_call call = {};
	u64 pa;

	/*
	 * Record the SVSM Calling Area address (CAA) if the guest is not
	 * running at VMPL0. The CA will be used to communicate with the
	 * SVSM to perform the SVSM services.
	 */
	if (!svsm_setup_ca(cc_info, rip_rel_ptr(&boot_svsm_ca_page)))
		return;

	/*
	 * It is very early in the boot and the kernel is running identity
	 * mapped but without having adjusted the pagetables to where the
	 * kernel was loaded (physbase), so the get the CA address using
	 * RIP-relative addressing.
	 */
	pa = (u64)rip_rel_ptr(&boot_svsm_ca_page);

	/*
	 * Switch over to the boot SVSM CA while the current CA is still 1:1
	 * mapped and thus addressable with VA == PA. There is no GHCB at this
	 * point so use the MSR protocol.
	 *
	 * SVSM_CORE_REMAP_CA call:
	 *   RAX = 0 (Protocol=0, CallID=0)
	 *   RCX = New CA GPA
	 */
	call.caa = (struct svsm_ca *)secrets->svsm_caa;
	call.rax = SVSM_CORE_CALL(SVSM_CORE_REMAP_CA);
	call.rcx = pa;

	if (svsm_call_msr_protocol(&call))
		sev_es_terminate(SEV_TERM_SET_LINUX, GHCB_TERM_SVSM_CA_REMAP_FAIL);

	boot_svsm_caa_pa = pa;
}

bool __init snp_init(struct boot_params *bp)
{
	struct cc_blob_sev_info *cc_info;

	if (!bp)
		return false;

	cc_info = find_cc_blob(bp);
	if (!cc_info)
		return false;

	if (cc_info->secrets_phys && cc_info->secrets_len == PAGE_SIZE)
		sev_secrets_pa = cc_info->secrets_phys;
	else
		return false;

	setup_cpuid_table(cc_info);

	svsm_setup(cc_info);

	/*
	 * The CC blob will be used later to access the secrets page. Cache
	 * it here like the boot kernel does.
	 */
	bp->cc_blob_address = (u32)(unsigned long)cc_info;

	return true;
}
