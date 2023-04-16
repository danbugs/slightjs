use std::alloc::{alloc, dealloc, Layout};
use std::io::{self, Read};
use std::ptr::copy_nonoverlapping;

use once_cell::sync::OnceCell;
use quickjs_wasm_rs::{Context, Value};
use send_wrapper::SendWrapper;
use slight_http_server::{get_js_req_arg, get_js_res_ret};

pub mod slight_http_server;
pub mod slight_keyvalue;

static CONTEXT: OnceCell<SendWrapper<Context>> = OnceCell::new();

fn console_log(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    let mut spaced = false;
    for arg in args {
        if spaced {
            print!(" ");
        } else {
            spaced = true;
        }
        print!("{}", arg.as_str()?);
    }
    println!();

    context.undefined_value()
}

fn from_utf8(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    let bytes = args[0].as_bytes()?.to_vec();
    let string = String::from_utf8(bytes).unwrap();
    context.value_from_str(&string)
}

fn do_init() -> anyhow::Result<()> {
    let mut script = String::new();
    io::stdin().read_to_string(&mut script)?;

    let context = Context::default();
    context.eval_global("script.js", &script)?;

    let global = context.global_object()?;
    let console = context.object_value()?;

    let entrypoint = global.get_property("_start")?;

    if !entrypoint.is_function() {
        panic!("expected function named \"_start\" defined in global scope");
    }

    // inject dependencies
    slight_keyvalue::inject_keyvalue_dependency(&context, &global)?;
    slight_http_server::inject_http_server_dependency(&context, &global)?;

    console.set_property("log", context.wrap_callback(console_log)?)?;
    global.set_property("console", console)?;
    global.set_property("fromUtf8", context.wrap_callback(from_utf8)?)?;

    CONTEXT.set(SendWrapper::new(context)).unwrap();
    Ok(())
}

#[export_name = "wizer.initialize"]
pub extern "C" fn init() {
    do_init().unwrap()
}

#[export_name = "_start"]
pub extern "C" fn start() {
    let context = CONTEXT.get().unwrap();
    let global = context.global_object().unwrap();
    let entrypoint = global.get_property("_start").unwrap();
    entrypoint.call(&global, &[]).unwrap();
}

#[export_name = "on-server-init"]
pub extern "C" fn on_server_init() -> i32 {
    let context = CONTEXT.get().unwrap();
    let global = context.global_object().unwrap();
    let entrypoint = global.get_property("_start").unwrap();
    entrypoint.call(&global, &[]).unwrap();
    1
}

#[export_name = "handle-hello"]
pub unsafe extern "C" fn handle_hello(
    arg0: i32,
    arg1: i32,
    arg2: i32,
    arg3: i32,
    arg4: i32,
    arg5: i32,
    arg6: i32,
    arg7: i32,
    arg8: i32,
    arg9: i32,
) -> i32 {
    let context = CONTEXT.get().unwrap();
    let global = context.global_object().unwrap();
    let entrypoint = global.get_property("handle_hello").unwrap();
    let req = get_js_req_arg(
        arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, &context,
    )
    .unwrap();

    let res = entrypoint.call(&global, &[req]).unwrap().as_str().unwrap().to_string();

    get_js_res_ret(res)
}

// Unlike C's realloc, zero-length allocations need not have
// unique addresses, so a zero-length allocation may be passed
// in and also requested, but it's ok to return anything that's
// non-zero to indicate success.
const ZERO_SIZE_ALLOCATION_PTR: *mut u8 = 1 as _;

/// 1. Allocate memory of new_size with alignment.
/// 2. If original_ptr != 0
///   a. copy min(new_size, original_size) bytes from original_ptr to new memory
///   b. de-allocate original_ptr
/// 3. return new memory ptr
///
/// # Safety
///
/// * `original_ptr` must be 0 or a valid pointer
/// * if `original_ptr` is not 0, it must be valid for reads of `original_size`
///   bytes
/// * if `original_ptr` is not 0, it must be properly aligned
/// * if `original_size` is not 0, it must match the `new_size` value provided
///   in the original `canonical_abi_realloc` call that returned `original_ptr`
#[export_name = "canonical_abi_realloc"]
pub unsafe extern "C" fn canonical_abi_realloc(
    original_ptr: *mut u8,
    original_size: usize,
    alignment: usize,
    new_size: usize,
) -> *mut std::ffi::c_void {
    assert!(new_size >= original_size);

    let new_mem = match new_size {
        0 => ZERO_SIZE_ALLOCATION_PTR,
        // this call to `alloc` is safe since `new_size` must be > 0
        _ => alloc(Layout::from_size_align(new_size, alignment).unwrap()),
    };

    if !original_ptr.is_null() && original_size != 0 {
        copy_nonoverlapping(original_ptr, new_mem, original_size);
        canonical_abi_free(original_ptr, original_size, alignment);
    }
    new_mem as _
}

/// Frees memory
///
/// # Safety
///
/// * `ptr` must denote a block of memory allocated by `canonical_abi_realloc`
/// * `size` and `alignment` must match the values provided in the original
///   `canonical_abi_realloc` call that returned `ptr`
#[export_name = "canonical_abi_free"]
pub unsafe extern "C" fn canonical_abi_free(ptr: *mut u8, size: usize, alignment: usize) {
    if size > 0 {
        dealloc(ptr, Layout::from_size_align(size, alignment).unwrap())
    };
}
