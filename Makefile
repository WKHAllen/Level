.PHONY: all build run test clean

all: build

build:
	cd backend && cargo tauri build

run:
	cd backend && cargo tauri dev

test:
	cargo test -- --nocapture

lint:
	cargo clippy -- -D warnings

clean:
	cargo clean
