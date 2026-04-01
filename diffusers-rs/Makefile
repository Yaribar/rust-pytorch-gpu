setup:
	bash setup.sh

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

build:
	cargo build --quiet

run:
	cargo run -- generate --prompt "A rusty robot walking on a sandy beach"

clean:
	cargo clean

all: format lint test build
