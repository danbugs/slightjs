# slightjs

This is an experimental project to work out the kinks of JS/TS + Wasm.

It is comprised of:
    - an `engine/`, which pre-initializes Wasm and injects necessary bindings into the JS context (i.e., w/ `quicjs-wasm-rs`),
    - a `cli/`, which inits the pre-initialized Wasm w/ actual guest code by passing it to the engine through `stdin`,
    - a `runtime/`, which is a thin wrapper around Wasmtime,
    - `ts-wit-bindgen`, which contains the grammar for the WIT language and a parser for it to create TS/JS bindings.