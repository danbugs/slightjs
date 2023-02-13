// !!! LET'S GET JS COMPILED TO WASM USING HOST FUNCTIONALITY FROM SPIDERLIGHTNING !!!

// #! wit/altered_keyvalue.wit
// ^^^ With a comment like this, we specify the wit file(s) make bindings for.

// We need to specify an entry point for Wasmtime...
function _start() {
    let kv = keyvalue.open("my-container");
    keyvalue.set(kv, "key", "Hello, JS Wasm!");
    console.log(keyvalue.get(kv, "key"));
}