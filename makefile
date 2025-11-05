example:
	cargo run -- parse queries/example.sql

run:
	cargo run

test:
	cargo test

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

release:
	cargo build --release

doc:
	cargo doc --open

clean:
	cargo clean

check: fmt clippy test

precommit: fmt clippy test