# YAI Unified Build System
CC = gcc

# Python embed flags (required by Engine main.c)
PYTHON_CONFIG ?= python3-config
PYTHON_CFLAGS := $(shell $(PYTHON_CONFIG) --includes 2>/dev/null)
PYTHON_LDFLAGS := $(shell $(PYTHON_CONFIG) --embed --ldflags 2>/dev/null || $(PYTHON_CONFIG) --ldflags 2>/dev/null)

CFLAGS = -Wall -Wextra -I./include -I../Kernel/include -I../Engine/include $(PYTHON_CFLAGS)

UNAME_S := $(shell uname -s)
LDFLAGS = -lpthread $(PYTHON_LDFLAGS)
ifeq ($(UNAME_S),Linux)
LDFLAGS += -lrt
endif

# Paths
API_DIR = .
ENGINE_DIR = ../Engine
KERNEL_DIR = ../Kernel

ENGINE_SRCS = $(ENGINE_DIR)/src/*.c

.PHONY: all kernel api clean run setup

all: kernel api

# 1. Compila il Kernel (Se presente come sorgente C)
kernel:
	@echo "Compilazione Kernel e componenti C..."
	$(CC) $(CFLAGS) $(ENGINE_SRCS) -o $(ENGINE_DIR)/yai-engine $(LDFLAGS)

# 2. Compila l'API Rust
api:
	@echo "Compilazione YAI (Rust)..."
	cargo build --release

# 3. Setup dei permessi (Critico per SHM)
setup:
	@echo "Configurazione permessi Shared Memory..."
	# Crea un file fittizio per forzare la creazione del segmento se necessario
	# sudo chmod 666 /dev/shm/yai_vault_* 2>/dev/run || true

# 4. Esecuzione Totale
run: all
	@echo "Lancio YAI Ecosystem..."
	./target/release/yai

clean:
	cargo clean
	rm -f $(ENGINE_DIR)/yai-engine
