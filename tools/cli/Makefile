# tools/cli/Makefile
CC ?= cc
CFLAGS ?= -O2 -Wall -Wextra -Werror -std=c11
LDFLAGS ?=
BIN = yai

# tools/cli/Makefile (aggiungi cmd_law.c e include dir)
SRC = \
  src/main.c \
  src/env.c \
  src/paths.c \
  src/envelope.c \
  src/rpc.c \
  src/fmt.c \
  src/cmd_root.c \
  src/cmd_law.c


INCLUDE = -Iinclude

all: $(BIN)

$(BIN): $(SRC)
	$(CC) $(CFLAGS) $(INCLUDE) -o $@ $(SRC) $(LDFLAGS)

clean:
	rm -f $(BIN)

.PHONY: all clean
