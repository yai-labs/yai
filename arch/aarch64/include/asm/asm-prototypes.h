/* SPDX-License-Identifier: GPL-2.0 */
#ifndef __ASM_PROTOTYPES_H
#define __ASM_PROTOTYPES_H
/*
 * CONFIG_MODVERSIONS requires a C declaration to generate the appropriate CRC
 * for each symbol. Since commit:
 *
 *   4efca4ed05cbdfd1 ("kbuild: modversions for EXPORT_SYMBOL() for asm")
 *
 * ... kbuild will automatically pick these up from <yai/asm-prototypes.h> and
 * feed this to genksyms when building assembly files.
 */
#include <yai/arm-smccc.h>

#include <yai/ftrace.h>
#include <yai/page.h>
#include <yai/string.h>
#include <yai/uaccess.h>

#include <yai/asm-prototypes.h>

long long __ashlti3(long long a, int b);
long long __ashrti3(long long a, int b);
long long __lshrti3(long long a, int b);

/*
 * This function uses a custom calling convention and cannot be called from C so
 * this prototype is not entirely accurate.
 */
void __hwasan_tag_mismatch(unsigned long addr, unsigned long access_info);

#endif /* __ASM_PROTOTYPES_H */
