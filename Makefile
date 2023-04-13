.PHONY: compile-slight-engine
compile-slight-engine:
	cargo build --package slightjs-engine --target wasm32-wasi

.PHONY: compile-guest-code
compile-guest-code:
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/index.js

.PHONY: testrun
testrun:
	slight -c ./examples/slightfile.toml run index.wasm