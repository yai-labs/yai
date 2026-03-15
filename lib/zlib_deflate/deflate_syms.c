// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/lib/zlib_deflate/deflate_syms.c
 *
 * Exported symbols for the deflate functionality.
 *
 */

#include <yai/module.h>
#include <yai/init.h>

#include <yai/zlib.h>

EXPORT_SYMBOL(zlib_deflate_workspacesize);
EXPORT_SYMBOL(zlib_deflate_dfltcc_enabled);
EXPORT_SYMBOL(zlib_deflate);
EXPORT_SYMBOL(zlib_deflateInit2);
EXPORT_SYMBOL(zlib_deflateEnd);
EXPORT_SYMBOL(zlib_deflateReset);
MODULE_DESCRIPTION("Data compression using the deflation algorithm");
MODULE_LICENSE("GPL");
