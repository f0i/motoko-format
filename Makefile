help:
	cat Makefile


clean:
	rm -rf ./release/

test:
	cargo test

test-release:
	cargo test --release

build:
	cargo build --target wasm32-unknown-unknown --features wasm --release

release: build
	mkdir -p release
	cp target/wasm32-unknown-unknown/release/dprint_plugin_motoko.wasm ./release/
