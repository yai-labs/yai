# yai/runtime-protocol/Makefile
CC ?= cc
AR ?= ar

# rpc_runtime.c include:
# - "rpc_runtime.h"  (canonico in yai-law runtime/include)
# - <yai_protocol_ids.h> (canonico in yai-law include)
CFLAGS ?= -O2 -g -Wall -Wextra \
  -I../deps/yai-law/contracts/protocol/runtime/include \
  -I../deps/yai-law/contracts/protocol/include

BUILD_DIR ?= ../build/runtime-protocol
OBJ_DIR := $(BUILD_DIR)/obj
LIB := $(BUILD_DIR)/libruntime_protocol.a

SRC := src/rpc_runtime.c
OBJ := $(OBJ_DIR)/rpc_runtime.o

.PHONY: all clean

all: $(LIB)

$(OBJ_DIR):
	mkdir -p $(OBJ_DIR)

$(OBJ): $(SRC) | $(OBJ_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(LIB): $(OBJ)
	mkdir -p $(BUILD_DIR)
	$(AR) rcs $@ $^

clean:
	rm -rf $(BUILD_DIR)
