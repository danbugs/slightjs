.PHONY: compile-slight-engine
compile-slight-engine:
	cargo build --package slightjs-engine --target wasm32-wasi

.PHONY: compile-keyvalue-example
compile-keyvalue-example:
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/keyvalue/index.js

.PHONY: compile-http-server-example
compile-http-server-example:
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/http-server/index.js

.PHONY: run-keyvalue-example
run-keyvalue-example:
	slight -c ./examples/keyvalue/slightfile.toml run index.wasm

.PHONY: run-http-server-example
run-http-server-example:
	slight -c ./examples/http-server/slightfile.toml run index.wasm

.PHONY: http-all-in-one
http-all-in-one:
	cargo build --package slightjs-engine --target wasm32-wasi
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/http-server/index.js
	slight -c ./examples/http-server/slightfile.toml run index.wasm