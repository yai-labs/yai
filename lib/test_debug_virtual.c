// SPDX-License-Identifier: GPL-2.0-only
#include <yai/kernel.h>
#include <yai/module.h>
#include <yai/export.h>
#include <yai/mm.h>
#include <yai/vmalloc.h>
#include <yai/slab.h>
#include <yai/sizes.h>
#include <yai/io.h>

#include <yai/page.h>
#ifdef CONFIG_MIPS
#include <yai/bootinfo.h>
#endif

struct foo {
	unsigned int bar;
};

static struct foo *foo;

static int __init test_debug_virtual_init(void)
{
	phys_addr_t pa;
	void *va;

	va = (void *)VMALLOC_START;
	pa = virt_to_phys(va);

	pr_info("PA: %pa for VA: 0x%lx\n", &pa, (unsigned long)va);

	foo = kzalloc_obj(*foo);
	if (!foo)
		return -ENOMEM;

	pa = virt_to_phys(foo);
	va = foo;
	pr_info("PA: %pa for VA: 0x%lx\n", &pa, (unsigned long)va);

	return 0;
}
module_init(test_debug_virtual_init);

static void __exit test_debug_virtual_exit(void)
{
	kfree(foo);
}
module_exit(test_debug_virtual_exit);

MODULE_LICENSE("GPL");
MODULE_DESCRIPTION("Test module for CONFIG_DEBUG_VIRTUAL");
