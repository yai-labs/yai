CC = gcc
CFLAGS = -Wall -Wextra -I./include -I../Bootstrap/include -O2
SRC = src/main.c src/fsm.c src/enforcement.c src/project_tree.c src/ids.c src/logger.c src/transport.c
OBJ = $(SRC:.c=.o)
TARGET = bin/ice-kernel

all: $(TARGET)

$(TARGET): $(OBJ)
	@mkdir -p bin
	$(CC) $(OBJ) -o $(TARGET)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f src/*.o bin/ice-kernel
