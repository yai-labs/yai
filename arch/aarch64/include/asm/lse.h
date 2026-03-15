/* SPDX-License-Identifier: GPL-2.0 */
#ifndef __ASM_LSE_H
#define __ASM_LSE_H

#include <yai/atomic_ll_sc.h>

#define __LSE_PREAMBLE	".arch_extension lse\n"

#include <yai/compiler_types.h>
#include <yai/export.h>
#include <yai/stringify.h>
#include <yai/alternative.h>
#include <yai/alternative-macros.h>
#include <yai/atomic_lse.h>
#include <yai/cpucaps.h>

#define __lse_ll_sc_body(op, ...)					\
({									\
	alternative_has_cap_likely(ARM64_HAS_LSE_ATOMICS) ?		\
		__lse_##op(__VA_ARGS__) :				\
		__ll_sc_##op(__VA_ARGS__);				\
})

/* In-line patching at runtime */
#define ARM64_LSE_ATOMIC_INSN(llsc, lse)				\
	ALTERNATIVE(llsc, __LSE_PREAMBLE lse, ARM64_HAS_LSE_ATOMICS)

#endif	/* __ASM_LSE_H */
