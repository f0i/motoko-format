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

test-external: release
	cd examples && ./test-external-repos.sh

reveiw-external: release
	cd examples && ./review-external-repos.sh

npm-publish: release
	npm install
	node setup.js 0.0.1
	npm run test
	npm publish --access public
