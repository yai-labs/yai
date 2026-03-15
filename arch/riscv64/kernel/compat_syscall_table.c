// SPDX-License-Identifier: GPL-2.0-only

#define __SYSCALL_COMPAT

#include <yai/compat.h>
#include <yai/syscalls.h>
#include <yai/mman-common.h>
#include <yai/syscalls.h>
#include <yai/syscall.h>

#define __SYSCALL_WITH_COMPAT(nr, native, compat) __SYSCALL(nr, compat)

#undef __SYSCALL
#define __SYSCALL(nr, call)	asmlinkage long __riscv_##call(const struct pt_regs *);
#include <yai/syscall_table_32.h>

#undef __SYSCALL
#define __SYSCALL(nr, call)      [nr] = __riscv_##call,

asmlinkage long compat_sys_rt_sigreturn(void);

void * const compat_sys_call_table[__NR_syscalls] = {
	[0 ... __NR_syscalls - 1] = __riscv_sys_ni_syscall,
#include <yai/syscall_table_32.h>
};
