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

# <sql>
.PHONY: compile-sql-example
compile-sql-example:
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/sql/index.js ./sql.wasm

.PHONY: run-sql-example
run-sql-example:
	slight -c ./examples/sql/slightfile.toml run sql.wasm -l

.PHONY: sql-all-in-one
sql-all-in-one:
	cargo build --package slightjs-engine --target wasm32-wasi
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/sql/index.js ./sql.wasm
	slight -c ./examples/sql/slightfile.toml run sql.wasm -l
# </sql>

# <http-client>
.PHONY: compile-http-client-example
compile-http-client-example:
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/http-client/index.js ./http-client.wasm

.PHONY: run-http-client-example
run-http-client-example:
	slight -c ./examples/http-client/slightfile.toml run http-client.wasm -l

.PHONY: http-client-all-in-one
http-client-all-in-one:
	cargo build --package slightjs-engine --target wasm32-wasi
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/http-client/index.js ./http-client.wasm
	slight -c ./examples/http-client/slightfile.toml run http-client.wasm -l
# </http-client>