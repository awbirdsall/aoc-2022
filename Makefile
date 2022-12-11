ci:
	cargo check
	cargo test
	cargo clippy -- -D warnings
	cargo fmt --check
