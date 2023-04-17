.PHONY: compile-slight-engine
compile-slight-engine:
	cargo build --package slightjs-engine --target wasm32-wasi

# <keyvalue>
.PHONY: compile-keyvalue-example
compile-keyvalue-example:
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/keyvalue/index.js

.PHONY: run-keyvalue-example
run-keyvalue-example:
	slight -c ./examples/keyvalue/slightfile.toml run index.wasm -l

.PHONY: keyvalue-all-in-one
keyvalue-all-in-one:
	cargo build --package slightjs-engine --target wasm32-wasi
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/keyvalue/index.js
	slight -c ./examples/keyvalue/slightfile.toml run index.wasm -l
# </keyvalue>

# <http-server>
.PHONY: compile-http-server-example
compile-http-server-example:
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/http-server/index.js

.PHONY: run-http-server-example
run-http-server-example:
	slight -c ./examples/http-server/slightfile.toml run index.wasm -l

.PHONY: http-all-in-one
http-all-in-one:
	cargo build --package slightjs-engine --target wasm32-wasi
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/http-server/index.js
	slight -c ./examples/http-server/slightfile.toml run index.wasm -l
# </http-server>

# <messaging>
.PHONY: compile-messaging-example
compile-messaging-example:
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/messaging/index.js

.PHONY: run-messaging-example
run-messaging-example:
	slight -c ./examples/messaging/slightfile.toml run index.wasm -l

.PHONY: messaging-all-in-one
messaging-all-in-one:
	cargo build --package slightjs-engine --target wasm32-wasi
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/messaging/index.js
	slight -c ./examples/messaging/slightfile.toml run index.wasm -l
# </messaging>