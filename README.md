# slightjs

This is an experimental project to make it possible to write JavaScript guest applications that make use of SpiderLightning capabilities.

It is comprised of:
- an `engine/`, which pre-initializes Wasm and injects slight dependencies into the JS context (i.e., w/ `quicjs-wasm-rs`), and
- a `cli/`, which inits the pre-initialized Wasm w/ actual guest code by passing it to the engine through `stdin`.

To try it out yourself, you can do: 
```bash
make compile-slight-engine
make compile-guest-code
make testrun
```

Here's a video demo: [![youtube-thumbnail](https://i.imgur.com/BCLkIr4.png)](https://youtu.be/O7fFuu569g0)

> Note: Examples ran on `slight 0.4.1 (d80ce5ba9 2023-04-12)`.