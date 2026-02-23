# ===============================
# YAI Mind (L3) Build
# ===============================

.PHONY: all clean run test

all:
	cargo build --release

run: all
	./target/release/yai

test:
	cargo test

clean:
	cargo clean
