// SPDX-License-Identifier: GPL-2.0
#include <yai/init.h>
#include <yai/types.h>
#include <yai/audit_arch.h>
#include <yai/unistd32.h>

unsigned compat_dir_class[] = {
#include <yai/audit_dir_write.h>
~0U
};

unsigned compat_read_class[] = {
#include <yai/audit_read.h>
~0U
};

unsigned compat_write_class[] = {
#include <yai/audit_write.h>
~0U
};

unsigned compat_chattr_class[] = {
#include <yai/audit_change_attr.h>
~0U
};

unsigned compat_signal_class[] = {
#include <yai/audit_signal.h>
~0U
};

int audit_classify_compat_syscall(int abi, unsigned syscall)
{
	switch (syscall) {
#ifdef __NR_open
	case __NR_open:
		return AUDITSC_OPEN;
#endif
#ifdef __NR_openat
	case __NR_openat:
		return AUDITSC_OPENAT;
#endif
#ifdef __NR_socketcall
	case __NR_socketcall:
		return AUDITSC_SOCKETCALL;
#endif
	case __NR_execve:
		return AUDITSC_EXECVE;
#ifdef __NR_openat2
	case __NR_openat2:
		return AUDITSC_OPENAT2;
#endif
	default:
		return AUDITSC_COMPAT;
	}
}
