# tools/cli/Makefile
CC ?= cc
CFLAGS ?= -O2 -Wall -Wextra -Werror -std=c11
LDFLAGS ?=
BIN = yai

SRC = \
  src/main.c \
  src/env.c \
  src/paths.c \
  src/envelope.c \
  src/rpc.c \
  src/fmt.c \
  src/cmd_root.c

INCLUDE = -Iinclude

all: $(BIN)

$(BIN): $(SRC)
	$(CC) $(CFLAGS) $(INCLUDE) -o $@ $(SRC) $(LDFLAGS)

clean:
	rm -f $(BIN)

.PHONY: all clean
