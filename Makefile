all: build test clippy fmt

.PHONY: build
build:
	cargo build --verbose

.PHONY: test
test:
	cargo test --verbose

.PHONY: clippy
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: format
format:
	cargo fmt

.PHONY: format-check
format-check:
	cargo fmt -- --check
