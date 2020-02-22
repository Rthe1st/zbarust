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

.PHONY: install-test-coverage
install-test-coverage:
	sudo apt-get install gettext libssl-dev\
	&& cargo install cargo-tarpaulin

.PHONY: test-coverage
test-coverage:
	cargo tarpaulin
