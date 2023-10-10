.PHONY: all build run demo test lint clean

all: build

build:
	cd backend && cargo tauri build

run:
	cd backend && cargo tauri dev

demo:
	cd backend && cargo tauri dev -- -- --demo

test:
	cargo test -- --nocapture

lint:
	cargo clippy -- -D warnings

clean:
	cargo clean
