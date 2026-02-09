CC = gcc
CFLAGS = -Wall -Wextra -I./include -O2

BOOT_SRC = src/boot/bootstrap.c src/boot/preboot.c src/bin/ice_boot_main.c
KERNEL_SRC = src/kernel/enforcement.c src/kernel/fsm.c src/kernel/ids.c src/kernel/logger.c src/kernel/project_tree.c src/kernel/transport.c src/bin/ice_kernel_main.c

BOOT_OBJ = $(BOOT_SRC:.c=.o)
KERNEL_OBJ = $(KERNEL_SRC:.c=.o)

BOOT_TARGET = bin/ice-boot
KERNEL_TARGET = bin/ice-kernel

all: boot kernel

boot: $(BOOT_TARGET)

kernel: $(KERNEL_TARGET)

$(BOOT_TARGET): $(BOOT_OBJ)
	@mkdir -p bin
	$(CC) $(BOOT_OBJ) -o $(BOOT_TARGET)

$(KERNEL_TARGET): $(KERNEL_OBJ)
	@mkdir -p bin
	$(CC) $(KERNEL_OBJ) -o $(KERNEL_TARGET)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f src/boot/*.o src/kernel/*.o src/bin/*.o bin/ice-boot bin/ice-kernel
