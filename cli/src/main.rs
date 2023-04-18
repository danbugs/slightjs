use std::{
    env,
    fs::{self, File},
    process::Command,
};

use anyhow::Result;
use wizer::Wizer;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let engine_path = &args[1];
    let js_path = &args[2];
    let output_file = &args[3];

    if env::var("JS_COMPILED").eq(&Ok("1".into())) {
        env::remove_var("JS_COMPILED");

        let wasm = fs::read(engine_path)?;

        let wasm = Wizer::new()
            .dir(".")
            .allow_wasi(true)?
            .inherit_stdio(true)
            .wasm_bulk_memory(true)
            .run(&wasm)?;

        fs::write(output_file, wasm)?;

        return Ok(());
    }

    env::set_var("JS_COMPILED", "1");

    let script = File::open(js_path)?;

    let self_cmd = std::env::current_exe()?;
    let status = Command::new(self_cmd)
        .arg(engine_path)
        .arg(js_path)
        .arg(output_file)
        .stdin(script)
        .status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("failed to compile"));
    }

    Ok(())
}
