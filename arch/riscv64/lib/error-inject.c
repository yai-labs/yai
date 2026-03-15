// SPDX-License-Identifier: GPL-2.0

#include <yai/error-injection.h>
#include <yai/kprobes.h>

void override_function_with_return(struct pt_regs *regs)
{
	instruction_pointer_set(regs, regs->ra);
}
NOKPROBE_SYMBOL(override_function_with_return);
