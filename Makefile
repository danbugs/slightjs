.PHONY: ts-binding-for-host-js
ts-binding-for-host-js:
	node ts-wit-bindgen/ts-wit-bindgen.js wit/host.wit

.PHONY: engage
engage:
	cargo build --package slightjs-engine --target wasm32-wasi
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/test.js
	cargo build --package slightjs-engine --target wasm32-wasi
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/test.js

.PHONY: run
run:
	cargo run --package slightjs-runtime -- ./index.wasm	
