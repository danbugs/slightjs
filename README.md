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

## Known Issues

- `slight` only imports dependencies that are specific in the `slightfile.toml` file. This doesn't work w/ the current engine's model that injects every dependency.

