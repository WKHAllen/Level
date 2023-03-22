.PHONY: all build run test clean

all: build

build:
	cd backend && cargo tauri build

run:
	cd backend && cargo tauri dev

test:
	cargo test -- --nocapture

clean:
	cargo clean
