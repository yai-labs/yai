// SPDX-License-Identifier: GPL-2.0
/*
 * decompress.c
 *
 * Detect the decompression method based on magic number
 */

#include <yai/decompress/generic.h>

#include <yai/decompress/bunzip2.h>
#include <yai/decompress/unlzma.h>
#include <yai/decompress/unxz.h>
#include <yai/decompress/inflate.h>
#include <yai/decompress/unlzo.h>
#include <yai/decompress/unlz4.h>
#include <yai/decompress/unzstd.h>

#include <yai/types.h>
#include <yai/string.h>
#include <yai/init.h>
#include <yai/printk.h>

#ifndef CONFIG_DECOMPRESS_GZIP
# define gunzip NULL
#endif
#ifndef CONFIG_DECOMPRESS_BZIP2
# define bunzip2 NULL
#endif
#ifndef CONFIG_DECOMPRESS_LZMA
# define unlzma NULL
#endif
#ifndef CONFIG_DECOMPRESS_XZ
# define unxz NULL
#endif
#ifndef CONFIG_DECOMPRESS_LZO
# define unlzo NULL
#endif
#ifndef CONFIG_DECOMPRESS_LZ4
# define unlz4 NULL
#endif
#ifndef CONFIG_DECOMPRESS_ZSTD
# define unzstd NULL
#endif

struct compress_format {
	unsigned char magic[2];
	const char *name;
	decompress_fn decompressor;
};

static const struct compress_format compressed_formats[] __initconst = {
	{ .magic = {0x1f, 0x8b}, .name = "gzip", .decompressor = gunzip },
	{ .magic = {0x1f, 0x9e}, .name = "gzip", .decompressor = gunzip },
	{ .magic = {0x42, 0x5a}, .name = "bzip2", .decompressor = bunzip2 },
	{ .magic = {0x5d, 0x00}, .name = "lzma", .decompressor = unlzma },
	{ .magic = {0xfd, 0x37}, .name = "xz", .decompressor = unxz },
	{ .magic = {0x89, 0x4c}, .name = "lzo", .decompressor = unlzo },
	{ .magic = {0x02, 0x21}, .name = "lz4", .decompressor = unlz4 },
	{ .magic = {0x28, 0xb5}, .name = "zstd", .decompressor = unzstd },
	{ /* sentinel */ }
};

decompress_fn __init decompress_method(const unsigned char *inbuf, long len,
				const char **name)
{
	const struct compress_format *cf;

	if (len < 2) {
		if (name)
			*name = NULL;
		return NULL;	/* Need at least this much... */
	}

	pr_debug("Compressed data magic: %#.2x %#.2x\n", inbuf[0], inbuf[1]);

	for (cf = compressed_formats; cf->name; cf++)
		if (!memcmp(inbuf, cf->magic, 2))
			break;

	if (name)
		*name = cf->name;
	return cf->decompressor;
}
