# <general>
.phony: build-slightjs-cli
build-slightjs-cli:
	cargo build --release --package slightjs-cli

.PHONY: compile-slight-engine
compile-slight-engine:
	cargo build --release --package slightjs-engine --target wasm32-wasi

.PHONY: improve
improve:
	cargo clippy --all-targets --all-features --workspace -- -D warnings
	cargo fmt --all -- --check

.PHONY: install-deps-linux
install-deps-linux:
	set -x
	curl -sS -L -O https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-15/wasi-sdk-15.0-linux.tar.gz
	tar xf wasi-sdk-15.0-linux.tar.gz
	sudo mkdir -p /opt/wasi-sdk
	sudo mv wasi-sdk-15.0/* /opt/wasi-sdk/
	sudo rm -rf wasi-sdk-*
	chmod +x /opt/wasi-sdk/bin/clang

.PHONY: install-deps-macos
install-deps-macos:
	set -x
	curl -sS -L -O https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-15/wasi-sdk-15.0-macos.tar.gz
	tar xf wasi-sdk-15.0-macos.tar.gz
	sudo mkdir -p /opt/wasi-sdk
	sudo mv wasi-sdk-15.0/* /opt/wasi-sdk/
	sudo rm -rf wasi-sdk-*
	chmod +x /opt/wasi-sdk/bin/clang

.PHONY: install-deps-win
install-deps-win:
	choco install make -y
	choco install wget -y
	wget -O wasi-sdk-15.0-mingw.tar.gz https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-15/wasi-sdk-15.0-mingw.tar.gz
	mkdir -p C:\wasi-sdk
	tar -xvzf wasi-sdk-15.0-mingw.tar.gz --strip-components=1 -C C:\wasi-sdk

.PHONY: prepare-release-linux
prepare-release-linux:
	tar -C target/ -czvf slightjs-linux-x86_64.tar.gz release/slight

.PHONY: prepare-release-win
prepare-release-win:
	tar -C target/ -czvf slightjs-windows-x86_64.tar.gz release/slight.exe

.PHONY: prepare-release-mac
prepare-release-mac:
	tar -C target/ -czvf slightjs-macos.tar.gz release/slight
# </general>

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

# <blob-store>
.PHONY: compile-blob-store-example
compile-blob-store-example:
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/blob-store/index.js ./blob-store.wasm

.PHONY: run-blob-store-example
run-blobs-tore-example:
	slight -c ./examples/blob-store/slightfile.toml run blob-store.wasm -l

.PHONY: blob-store-all-in-one
blob-store-all-in-one:
	cargo build --package slightjs-engine --target wasm32-wasi
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/blob-store/index.js ./blob-store.wasm
	slight -c ./examples/blob-store/slightfile.toml run blob-store.wasm -l
# </blob-store>

# <configs>
.PHONY: compile-configs-example
compile-configs-example:
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/configs/index.js ./configs.wasm

.PHONY: run-configs-example
run-configs-example:
	slight -c ./examples/configs/slightfile.toml run configs.wasm -l

.PHONY: configs-all-in-one
configs-all-in-one:
	cargo build --package slightjs-engine --target wasm32-wasi
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/configs/index.js ./configs.wasm
	slight -c ./examples/configs/slightfile.toml run configs.wasm -l
# </configs>

# <distributed-locking>
.PHONY: compile-distributed-locking-example
compile-distributed-locking-example:
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/distributed-locking/index.js ./distributed-locking.wasm

.PHONY: run-distributed-locking-example
run-distributed-locking-example:
	slight -c ./examples/distributed-locking/slightfile.toml run distributed-locking.wasm -l &
	slight -c ./examples/distributed-locking/slightfile.toml run distributed-locking.wasm -l

.PHONY: distributed-locking-all-in-one
distributed-locking-all-in-one:
	cargo build --package slightjs-engine --target wasm32-wasi
	cargo run --package slightjs-cli -- ./target/wasm32-wasi/debug/slightjs_engine.wasm ./examples/distributed-locking/index.js ./distributed-locking.wasm
	slight -c ./examples/distributed-locking/slightfile.toml run distributed-locking.wasm -l &
	slight -c ./examples/distributed-locking/slightfile.toml run distributed-locking.wasm -l
# </distributed-locking>