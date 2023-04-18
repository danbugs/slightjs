.PHONY: compile-slight-engine
compile-slight-engine:
	cargo build --package slightjs-engine --target wasm32-wasi

# <keyvalue>
.PHONY: compile-keyvalue-example
compile-keyvalue-example:
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/keyvalue/index.js ./keyvalue.wasm

.PHONY: run-keyvalue-example
run-keyvalue-example:
	slight -c ./examples/keyvalue/slightfile.toml run keyvalue.wasm -l

.PHONY: keyvalue-all-in-one
keyvalue-all-in-one:
	cargo build --package slightjs-engine --target wasm32-wasi
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/keyvalue/index.js ./keyvalue.wasm
	slight -c ./examples/keyvalue/slightfile.toml run keyvalue.wasm -l
# </keyvalue>

# <http-server>
.PHONY: compile-http-server-example
compile-http-server-example:
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/http-server/index.js ./http-server.wasm

.PHONY: run-http-server-example
run-http-server-example:
	slight -c ./examples/http-server/slightfile.toml run http-server.wasm -l

.PHONY: http-all-in-one
http-all-in-one:
	cargo build --package slightjs-engine --target wasm32-wasi
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/http-server/index.js ./http-server.wasm
	slight -c ./examples/http-server/slightfile.toml run http-server.wasm -l
# </http-server>

# <messaging>
.PHONY: compile-messaging-example
compile-messaging-example:
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/messaging/index.js ./messaging.wasm

.PHONY: run-messaging-example
run-messaging-example:
	slight -c ./examples/messaging/slightfile.toml run messaging.wasm -l

.PHONY: messaging-all-in-one
messaging-all-in-one:
	cargo build --package slightjs-engine --target wasm32-wasi
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/messaging/index.js ./messaging.wasm
	slight -c ./examples/messaging/slightfile.toml run messaging.wasm -l
# </messaging>

# <app>
.PHONY: compile-app-example
compile-app-example:
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/app/backend/index.js ./app.wasm

.PHONY: run-app-example
run-app-example:
	slight -c ./examples/app/backend/slightfile.toml run app.wasm -l

.PHONY: app-all-in-one
app-all-in-one:
	cargo build --package slightjs-engine --target wasm32-wasi
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/app/backend/index.js ./app.wasm
	slight -c ./examples/app/backend/slightfile.toml run app.wasm -l
# </app>