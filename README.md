# slightjs

This is an experimental project to make it possible to write JavaScript guest applications that make use of SpiderLightning capabilities.

It is comprised of:
- an `engine/`, which pre-initializes Wasm and injects slight dependencies into the JS context (i.e., w/ `quickjs-wasm-rs`), and
- a `cli/`, which inits the pre-initialized Wasm w/ actual guest code by passing it to the engine through `stdin`.

To try it out yourself, you can do: 
```bash
make keyvalue-all-in-one
```

Here's a video demo: [![youtube-thumbnail](./docs/imgs/slightjs_app_demo_thumbnail.png)](https://youtu.be/dTyx3UTJdUI)

> Note: Examples ran on a modified version of `slight` still to be merged into `main` (see more, here: https://github.com/deislabs/spiderlightning/pull/386).

## Installation

### UNIX

```sh
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/danbugs/slightjs/main/install.sh)"
```

### Windows

```sh
iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/danbugs/slightjs/main/install.ps1'))
```

## Engine Download

```sh
curl -O slightjs_engine.wasm https://raw.githubusercontent.com/danbugs/slightjs/main/slightjs_engine.wasm
```