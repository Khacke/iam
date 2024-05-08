build:
	cargo build --release

test:
	cargo test

main:
	RUST_LOG=debug cargo run --release -p manager
	