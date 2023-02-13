# slightjs

This is an experimental project to work out the kinks of JS/TS + Wasm.

It is comprised of:
    - an `engine/`, which pre-initializes Wasm and injects necessary bindings into the JS context (i.e., w/ `quicjs-wasm-rs`),
    - a `cli/`, which inits the pre-initialized Wasm w/ actual guest code by passing it to the engine through `stdin`,
    - a `runtime/`, which is a thin wrapper around Wasmtime for testing purposes, and
    - `ts-wit-bindgen`, which contains the grammar for the WIT language and a parser for it to create TS/JS bindings.

To try it out yourself, you can do: `make compile-js && make run`. This will compile our `host-test.js` file and make use of the `host.wit` interface to print out: '42'.

If you have [`slight`](https://github.com/deislabs/spiderlightning) installed in your machine, you can also run the slight example, like: `make compile-js-slight && slight -c examples/slightfile.toml run -m index.wasm`.

For a video example, see this:

[![img](https://i.imgur.com/nM6Z6SY.png)](https://youtu.be/EHzDrTSOEi0)

> Note: This work is based on WIT v0.2.0 – It does not currently support `interface`s, and `world`s.