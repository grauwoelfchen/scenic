check:
	@cargo check --all --verbose

fmt:
	@cargo fmt --all -- --check

lint:
	@cargo clippy --all-targets

.PHONY: check fmt lint

vet: check fmt lint

.PHONY: vet

test:
	@cargo test --tests -- --nocapture

clean:
	@cargo clean
	@rm -f dst/*.pdf

build:
	cargo build

.PHONY: build clean test
