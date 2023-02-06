use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::{WasiCtxBuilder, WasiCtx};

fn main() -> Result<()> {
    println!("Compiling module...");
    let engine = Engine::default();

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    linker.func_wrap("host", "double", |x: i32| x * 2)?;

    let args = std::env::args().collect::<Vec<String>>();
    let index_wasm = &args[1];
    let module = Module::from_file(&engine, index_wasm)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();

    println!("Initializing...");
    let mut store = Store::new(&engine, wasi);
    let instance = linker.instantiate(&mut store, &module)?;

    println!("Starting...");
    instance
        .get_typed_func::<(), (), &mut wasmtime::Store<WasiCtx>>(&mut store, "_start")?
        .call(&mut store, ())?;

    println!("Done.");
    Ok(())
}
