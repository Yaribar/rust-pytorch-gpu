install:
	python3 -m pip install --upgrade pip \
		&& pip install -r requirements.txt

rust-version:
	rustc --version
	cargo --version
	clippy-driver --version

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

clean:
	#cargo install cargo-cache
	#cargo cache -a
	#rm -rf Cargo.lock
	cargo clean

run:
	cargo run 

all: format lint test run