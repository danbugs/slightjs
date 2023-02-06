use anyhow::Result;
use quickjs_wasm_rs::{Context, Value};

pub fn dynamic_context_injection(_context: &Context, _global: &Value) -> Result<()> {
    Ok(())
}
