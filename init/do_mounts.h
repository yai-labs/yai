/* SPDX-License-Identifier: GPL-2.0 */
#include <yai/kernel.h>
#include <yai/blkdev.h>
#include <yai/init.h>
#include <yai/syscalls.h>
#include <yai/unistd.h>
#include <yai/slab.h>
#include <yai/mount.h>
#include <yai/major.h>
#include <yai/root_dev.h>
#include <yai/init_syscalls.h>
#include <yai/task_work.h>
#include <yai/file.h>

void  mount_root_generic(char *name, char *pretty_name, int flags);
void  mount_root(char *root_device_name);
extern int root_mountflags;

static inline __init int create_dev(char *name, dev_t dev)
{
	init_unlink(name);
	return init_mknod(name, S_IFBLK | 0600, new_encode_dev(dev));
}

#ifdef CONFIG_BLK_DEV_RAM
int __init rd_load_image(void);
#else
static inline int rd_load_image(void) { return 0; }
#endif

#ifdef CONFIG_BLK_DEV_INITRD
void __init initrd_load(void);
#else
static inline void initrd_load(void) { }
#endif

/* Ensure that async file closing finished to prevent spurious errors. */
static inline void init_flush_fput(void)
{
	flush_delayed_fput();
	task_work_run();
}
