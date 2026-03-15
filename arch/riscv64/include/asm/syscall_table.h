#include <yai/bitsperlong.h>

#if __BITS_PER_LONG == 64
#include <yai/syscall_table_64.h>
#else
#include <yai/syscall_table_32.h>
#endif
