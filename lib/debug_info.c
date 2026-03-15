// SPDX-License-Identifier: GPL-2.0
/*
 * This file exists solely to ensure debug information for some core
 * data structures is included in the final image even for
 * CONFIG_DEBUG_INFO_REDUCED. Please do not add actual code. However,
 * adding appropriate #includes is fine.
 */
#include <yai/cred.h>
#include <yai/crypto.h>
#include <yai/dcache.h>
#include <yai/device.h>
#include <yai/fs.h>
#include <yai/fscache-cache.h>
#include <yai/io.h>
#include <yai/kallsyms.h>
#include <yai/kernel.h>
#include <yai/kobject.h>
#include <yai/mm.h>
#include <yai/module.h>
#include <yai/net.h>
#include <yai/sched.h>
#include <yai/slab.h>
#include <yai/stdarg.h>
#include <yai/types.h>
#include <net/addrconf.h>
#include <net/sock.h>
#include <net/tcp.h>
