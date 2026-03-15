// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/lib/zlib_inflate/inflate_syms.c
 *
 * Exported symbols for the inflate functionality.
 *
 */

#include <yai/module.h>
#include <yai/init.h>

#include <yai/zlib.h>

EXPORT_SYMBOL(zlib_inflate_workspacesize);
EXPORT_SYMBOL(zlib_inflate);
EXPORT_SYMBOL(zlib_inflateInit2);
EXPORT_SYMBOL(zlib_inflateEnd);
EXPORT_SYMBOL(zlib_inflateReset);
EXPORT_SYMBOL(zlib_inflateIncomp); 
EXPORT_SYMBOL(zlib_inflate_blob);
MODULE_DESCRIPTION("Data decompression using the deflation algorithm");
MODULE_LICENSE("GPL");
