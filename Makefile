check:
	@cargo check --all --verbose

fmt:
	@cargo fmt --all -- --check

lint:
	@cargo clippy --all-targets

test:
	@cargo test --tests -- --nocapture

clean:
	@cargo clean
	@rm -f dst/*.pdf

build:
	cargo build

.PHONY: build clean check fmt lint test
