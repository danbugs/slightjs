use quickjs_wasm_rs::{Context, Value};

use crate::slight_http_types::Response;

#[link(wasm_import_module = "http-server")]
extern "C" {
    #[cfg_attr(target_arch = "wasm32", link_name = "router::new")]
    fn new(_: i32);

    #[cfg_attr(target_arch = "wasm32", link_name = "router::new-with-base")]
    fn new_with_base(_: i32, _: i32, _: i32);

    #[cfg_attr(target_arch = "wasm32", link_name = "router::get")]
    fn get1(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32);

    #[cfg_attr(target_arch = "wasm32", link_name = "router::post")]
    fn post(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32);

    #[cfg_attr(target_arch = "wasm32", link_name = "router::put")]
    fn put(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32);

    #[cfg_attr(target_arch = "wasm32", link_name = "router::delete")]
    fn delete1(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32);

    #[cfg_attr(target_arch = "wasm32", link_name = "server::serve")]
    fn serve(_: i32, _: i32, _: i32, _: i32);

    #[cfg_attr(target_arch = "wasm32", link_name = "server::stop")]
    fn stop(_: i32, _: i32);
}

pub fn router_new(context: &Context, _this: &Value, _: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let ptr0 = __HTTP_SERVER_RET_AREA.0.as_mut_ptr() as i32;
        new(ptr0);
        match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => context.value_from_i32(*((ptr0 + 4) as *const i32)),
            1 => context.value_from_str(&HttpRouterError(ptr0)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

/// create a new HTTP router
pub fn router_new_with_base(
    context: &Context,
    _this: &Value,
    args: &[Value],
) -> anyhow::Result<Value> {
    unsafe {
        let vec0 = args[0].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __HTTP_SERVER_RET_AREA.0.as_mut_ptr() as i32;
        new_with_base(ptr0, len0, ptr1);
        match i32::from(*((ptr1 + 0) as *const u8)) {
            0 => context.value_from_i32(*((ptr1 + 4) as *const i32)),
            1 => context.value_from_str(&HttpRouterError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

/// register a HTTP GET route
pub fn router_get(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let vec0 = args[1].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let vec1 = args[2].as_str().unwrap().to_string();
        let ptr1 = vec1.as_ptr() as i32;
        let len1 = vec1.len() as i32;
        let ptr2 = __HTTP_SERVER_RET_AREA.0.as_mut_ptr() as i32;
        get1(self0, ptr0, len0, ptr1, len1, ptr2);
        match i32::from(*((ptr2 + 0) as *const u8)) {
            0 => context.value_from_i32(*((ptr2 + 4) as *const i32)),
            1 => context.value_from_str(&HttpRouterError(ptr2)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

/// register a HTTP PUT route
pub fn router_put(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let vec0 = args[1].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let vec1 = args[2].as_str().unwrap().to_string();
        let ptr1 = vec1.as_ptr() as i32;
        let len1 = vec1.len() as i32;
        let ptr2 = __HTTP_SERVER_RET_AREA.0.as_mut_ptr() as i32;
        put(self0, ptr0, len0, ptr1, len1, ptr2);
        match i32::from(*((ptr2 + 0) as *const u8)) {
            0 => context.value_from_i32(*((ptr2 + 4) as *const i32)),
            1 => context.value_from_str(&HttpRouterError(ptr2)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

/// register a HTTP POST route
pub fn router_post(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0 = args[0].as_i32_unchecked();
        let vec0 = args[1].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let vec1 = args[2].as_str().unwrap().to_string();
        let ptr1 = vec1.as_ptr() as i32;
        let len1 = vec1.len() as i32;
        let ptr2 = __HTTP_SERVER_RET_AREA.0.as_mut_ptr() as i32;
        post(self0, ptr0, len0, ptr1, len1, ptr2);
        match i32::from(*((ptr2 + 0) as *const u8)) {
            0 => context.value_from_i32(*((ptr2 + 4) as *const i32)),
            1 => context.value_from_str(&HttpRouterError(ptr2)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

/// register a HTTP DELETE route
pub fn router_delete(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0 = args[0].as_i32_unchecked();
        let vec0 = args[1].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let vec1 = args[2].as_str().unwrap().to_string();
        let ptr1 = vec1.as_ptr() as i32;
        let len1 = vec1.len() as i32;
        let ptr2 = __HTTP_SERVER_RET_AREA.0.as_mut_ptr() as i32;
        delete1(self0, ptr0, len0, ptr1, len1, ptr2);
        match i32::from(*((ptr2 + 0) as *const u8)) {
            0 => context.value_from_i32(*((ptr2 + 4) as *const i32)),
            1 => context.value_from_str(&HttpRouterError(ptr2)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

/// create a new HTTP server and serve the given router
pub fn server_serve(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let vec0 = args[0].as_str().unwrap().to_string();
        let vec1 = args[1].as_i32_unchecked();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __HTTP_SERVER_RET_AREA.0.as_mut_ptr() as i32;
        serve(ptr0, len0, vec1, ptr1);
        match i32::from(*((ptr1 + 0) as *const u8)) {
            0 => context.value_from_i32(*((ptr1 + 4) as *const i32)),
            1 => context.value_from_str(&HttpRouterError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

/// stop the server
pub fn server_stop(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0 = args[0].as_i32_unchecked();
        let ptr0 = __HTTP_SERVER_RET_AREA.0.as_mut_ptr() as i32;
        stop(self0, ptr0);
        match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => context.null_value(),
            1 => context.value_from_str(&HttpRouterError(ptr0)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

#[allow(non_snake_case)]
fn HttpRouterError(ptr: i32) -> String {
    unsafe {
        match i32::from(*((ptr + 4) as *const u8)) {
            0 => {
                let len1 = *((ptr + 12) as *const i32) as usize;

                String::from_utf8(Vec::from_raw_parts(
                    *((ptr + 8) as *const i32) as *mut _,
                    len1,
                    len1,
                ))
                .unwrap()
            }
            1 => {
                let len2 = *((ptr + 12) as *const i32) as usize;

                String::from_utf8(Vec::from_raw_parts(
                    *((ptr + 8) as *const i32) as *mut _,
                    len2,
                    len2,
                ))
                .unwrap()
            }
            2 => {
                let len3 = *((ptr + 12) as *const i32) as usize;

                String::from_utf8(Vec::from_raw_parts(
                    *((ptr + 8) as *const i32) as *mut _,
                    len3,
                    len3,
                ))
                .unwrap()
            }
            3 => i32::from(*((ptr + 8) as *const u16)).to_string(),
            4 => {
                let len4 = *((ptr + 12) as *const i32) as usize;

                String::from_utf8(Vec::from_raw_parts(
                    *((ptr + 8) as *const i32) as *mut _,
                    len4,
                    len4,
                ))
                .unwrap()
            }
            _ => panic!("invalid enum discriminant"),
        }
    }
}

#[repr(align(4))]
struct __HttpServerRetArea([u8; 16]);
static mut __HTTP_SERVER_RET_AREA: __HttpServerRetArea = __HttpServerRetArea([0; 16]);

pub fn inject_http_server_dependency(context: &Context, global: &Value) -> anyhow::Result<()> {
    let router = context.object_value()?;
    router.set_property("new", context.wrap_callback(router_new)?)?;
    router.set_property("get", context.wrap_callback(router_get)?)?;
    router.set_property("post", context.wrap_callback(router_post)?)?;
    router.set_property("put", context.wrap_callback(router_put)?)?;
    router.set_property("delete", context.wrap_callback(router_delete)?)?;
    global.set_property("router", router)?;

    let server = context.object_value()?;
    server.set_property("serve", context.wrap_callback(server_serve)?)?;
    server.set_property("stop", context.wrap_callback(server_stop)?)?;
    global.set_property("server", server)?;

    Ok(())
}

pub unsafe fn get_js_req_arg(
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
    context: &Context,
) -> anyhow::Result<Value> {
    let len0 = arg2 as usize;
    let base3 = arg3;
    let len3 = arg4;
    let mut result3 = Vec::with_capacity(len3 as usize);
    for i in 0..len3 {
        let base = base3 + i * 16;
        result3.push({
            let len1 = *((base + 4) as *const i32) as usize;
            let len2 = *((base + 12) as *const i32) as usize;
            (
                String::from_utf8(Vec::from_raw_parts(
                    *((base + 0) as *const i32) as *mut _,
                    len1,
                    len1,
                ))
                .unwrap(),
                String::from_utf8(Vec::from_raw_parts(
                    *((base + 8) as *const i32) as *mut _,
                    len2,
                    len2,
                ))
                .unwrap(),
            )
        });
    }
    if len3 != 0 {
        std::alloc::dealloc(
            base3 as *mut _,
            std::alloc::Layout::from_size_align_unchecked((len3 as usize) * 16, 4),
        );
    }
    let base6 = arg5;
    let len6 = arg6;
    let mut result6 = Vec::with_capacity(len6 as usize);
    for i in 0..len6 {
        let base = base6 + i * 16;
        result6.push({
            let len4 = *((base + 4) as *const i32) as usize;
            let len5 = *((base + 12) as *const i32) as usize;
            (
                String::from_utf8(Vec::from_raw_parts(
                    *((base + 0) as *const i32) as *mut _,
                    len4,
                    len4,
                ))
                .unwrap(),
                String::from_utf8(Vec::from_raw_parts(
                    *((base + 8) as *const i32) as *mut _,
                    len5,
                    len5,
                ))
                .unwrap(),
            )
        });
    }
    if len6 != 0 {
        std::alloc::dealloc(
            base6 as *mut _,
            std::alloc::Layout::from_size_align_unchecked((len6 as usize) * 16, 4),
        );
    }

    let request = context.object_value()?;
    request.set_property(
        "method",
        match arg0 {
            0 => context.value_from_str("GET")?,
            1 => context.value_from_str("POST")?,
            2 => context.value_from_str("PUT")?,
            3 => context.value_from_str("DELETE")?,
            _ => panic!("not supported HTTP method"),
        },
    )?;
    request.set_property(
        "uri",
        context.value_from_str(
            &String::from_utf8(Vec::from_raw_parts(arg1 as *mut _, len0, len0)).unwrap(),
        )?,
    )?;
    request.set_property(
        "headers",
        context.value_from_str(&serde_json::to_string(&result3).unwrap())?,
    )?;
    request.set_property(
        "params",
        context.value_from_str(&serde_json::to_string(&result6).unwrap())?,
    )?;
    request.set_property(
        "body",
        match arg7 {
            0 => context.null_value()?,
            1 => {
                let len7 = arg9 as usize;
                context.value_from_str(
                    &String::from_utf8(Vec::from_raw_parts(arg8 as *mut _, len7, len7)).unwrap(),
                )?
            }
            _ => {
                panic!("invalid enum discriminant")
            }
        },
    )?;

    Ok(request)
}

pub unsafe fn get_js_res_ret(result: String) -> i32 {
    let ptr8 = 0;

    *((ptr8 + 0) as *mut u8) = (0i32) as u8;

    let Response {
        status: status9,
        headers: headers9,
        body: body9,
    } = serde_json::from_str(&result).unwrap();

    *((ptr8 + 4) as *mut u16) = (wit_bindgen_rust::rt::as_i32(status9)) as u16;

    match headers9 {
        Some(e) => {
            *((ptr8 + 8) as *mut u8) = (1i32) as u8;
            let vec13 = e;
            let len13 = vec13.len() as i32;
            let layout13 = core::alloc::Layout::from_size_align_unchecked(vec13.len() * 16, 4);
            let result13 = if layout13.size() != 0 {
                let ptr = std::alloc::alloc(layout13);
                if ptr.is_null() {
                    std::alloc::handle_alloc_error(layout13);
                }
                ptr
            } else {
                std::ptr::null_mut()
            };
            for (i, e) in vec13.into_iter().enumerate() {
                let base = result13 as i32 + (i as i32) * 16;
                {
                    let (t10_0, t10_1) = e;
                    let vec11 = (t10_0.into_bytes()).into_boxed_slice();
                    let ptr11 = vec11.as_ptr() as i32;
                    let len11 = vec11.len() as i32;
                    core::mem::forget(vec11);
                    *((base + 4) as *mut i32) = len11;
                    *((base + 0) as *mut i32) = ptr11;
                    let vec12 = (t10_1.into_bytes()).into_boxed_slice();
                    let ptr12 = vec12.as_ptr() as i32;
                    let len12 = vec12.len() as i32;
                    core::mem::forget(vec12);
                    *((base + 12) as *mut i32) = len12;
                    *((base + 8) as *mut i32) = ptr12;
                }
            }
            *((ptr8 + 16) as *mut i32) = len13;
            *((ptr8 + 12) as *mut i32) = result13 as i32;
        }
        None => {
            *((ptr8 + 8) as *mut u8) = (0i32) as u8;
        }
    };
    match body9 {
        Some(e) => {
            *((ptr8 + 20) as *mut u8) = (1i32) as u8;
            let vec14 = (e).into_bytes().into_boxed_slice();
            let ptr14 = vec14.as_ptr() as i32;
            let len14 = vec14.len() as i32;
            core::mem::forget(vec14);
            *((ptr8 + 28) as *mut i32) = len14;
            *((ptr8 + 24) as *mut i32) = ptr14;
        }
        None => {
            *((ptr8 + 20) as *mut u8) = (0i32) as u8;
        }
    };

    ptr8
}
