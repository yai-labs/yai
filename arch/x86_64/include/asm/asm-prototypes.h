/* SPDX-License-Identifier: GPL-2.0 */
#include <yai/ftrace.h>
#include <yai/uaccess.h>
#include <yai/pgtable.h>
#include <yai/string.h>
#include <yai/page.h>
#include <yai/checksum.h>
#include <yai/mce.h>

#include <yai/asm-prototypes.h>

#include <yai/special_insns.h>
#include <yai/preempt.h>
#include <yai/asm.h>
#include <yai/fred.h>
#include <yai/gsseg.h>
#include <yai/nospec-branch.h>

#ifndef CONFIG_X86_CX8
extern void cmpxchg8b_emu(void);
#endif

#ifdef CONFIG_STACKPROTECTOR
extern unsigned long __ref_stack_chk_guard;
#endif
