/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */
#ifndef __ASM_BITOPS_H
#define __ASM_BITOPS_H

#include <yai/compiler.h>

#ifndef _LINUX_BITOPS_H
#error only <yai/bitops.h> can be included directly
#endif

#include <yai/bitops/builtin-__ffs.h>
#include <yai/bitops/builtin-ffs.h>
#include <yai/bitops/builtin-__fls.h>
#include <yai/bitops/builtin-fls.h>

#include <yai/bitops/ffz.h>
#include <yai/bitops/fls64.h>

#include <yai/bitops/sched.h>
#include <yai/bitops/hweight.h>

#include <yai/bitops/atomic.h>
#include <yai/bitops/lock.h>
#include <yai/bitops/non-atomic.h>
#include <yai/bitops/le.h>
#include <yai/bitops/ext2-atomic-setbit.h>

#endif /* __ASM_BITOPS_H */
