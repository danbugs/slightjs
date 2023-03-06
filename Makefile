.PHONY: ts-binding-for-host-js
ts-binding-for-host-js:
	node ts-wit-bindgen/ts-wit-bindgen.js ../wit/altered_keyvalue.wit

.PHONY: compile-js
compile-js:
	cargo build --package slightjs-engine --target wasm32-wasi
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/host-test.js
	cargo build --package slightjs-engine --target wasm32-wasi
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/host-test.js

.PHONY: run
run:
	cargo run --package slightjs-runtime -- ./index.wasm	

.PHONY: compile-js-slight
compile-js-slight:
	cargo build --package slightjs-engine --target wasm32-wasi
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/slight-test.js
	cargo build --package slightjs-engine --target wasm32-wasi
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/slight-test.js
