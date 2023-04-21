use quickjs_wasm_rs::{Context, Value};

use crate::slight_http_types::{Method, Request, Response};

#[link(wasm_import_module = "http-client")]
extern "C" {
    #[cfg_attr(target_arch = "wasm32", link_name = "request")]
    fn request(
        _: i32,
        _: i32,
        _: i32,
        _: i32,
        _: i32,
        _: i32,
        _: i32,
        _: i32,
        _: i32,
        _: i32,
        _: i32,
    );
}

pub fn http_client_request(
    context: &Context,
    _this: &Value,
    args: &[Value],
) -> anyhow::Result<Value> {
    unsafe {
        let Request {
            method: method0,
            uri: uri0,
            headers: headers0,
            params: params0,
            body: body0,
        } = {
            let obj = args[0].clone();
            let method = match obj.get_property("method")?.as_str()? {
                "GET" => Method::Get,
                "POST" => Method::Post,
                "PUT" => Method::Put,
                "DELETE" => Method::Delete,
                "PATCH" => Method::Patch,
                "HEAD" => Method::Head,
                "OPTIONS" => Method::Options,
                _ => panic!("invalid method"),
            };
            let uri = obj.get_property("uri")?.as_str()?.to_string();
            let headers = {
                let headers = obj.get_property("headers")?;
                let headers_len = headers.get_property("length")?.as_i32_unchecked();
                let mut result = Vec::with_capacity(headers_len as usize);
                for i in 0..headers_len {
                    let i_elem = headers.get_indexed_property(i.try_into()?)?;
                    let name = i_elem.get_property("name")?.as_str()?.to_string();
                    let value = i_elem.get_property("value")?.as_str()?.to_string();
                    result.push((name, value));
                }
                result
            };

            let params = {
                let params = obj.get_property("params")?;
                let params_len = params.get_property("length")?.as_i32_unchecked();
                let mut result = Vec::with_capacity(params_len as usize);
                for i in 0..params_len {
                    let i_elem = params.get_indexed_property(i.try_into()?)?;
                    let name = i_elem.get_property("name")?.as_str()?.to_string();
                    let value = i_elem.get_property("value")?.as_str()?.to_string();
                    result.push((name, value));
                }
                result
            };

            let body: Option<Vec<u8>> = {
                let body = obj.get_property("body")?;
                if body.is_null() {
                    None
                } else {
                    let body_len = body.get_property("length")?.as_i32_unchecked();
                    let mut result = Vec::with_capacity(body_len as usize);
                    for i in 0..body_len {
                        let i_elem = body.get_indexed_property(i.try_into()?)?;
                        let value = i_elem.as_i32_unchecked();
                        result.push(value as u8);
                    }
                    Some(result.clone())
                }
            };

            Request {
                method,
                uri,
                headers,
                params,
                body,
            }
        };
        let vec1 = uri0;
        let ptr1 = vec1.as_ptr() as i32;
        let len1 = vec1.len() as i32;
        let vec5 = headers0;
        let len5 = vec5.len() as i32;
        let layout5 = core::alloc::Layout::from_size_align_unchecked(vec5.len() * 16, 4);
        let result5 = if layout5.size() != 0 {
            let ptr = std::alloc::alloc(layout5);
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout5);
            }
            ptr
        } else {
            std::ptr::null_mut()
        };
        for (i, e) in vec5.into_iter().enumerate() {
            let base = result5 as i32 + (i as i32) * 16;
            {
                let (t2_0, t2_1) = e;
                let vec3 = t2_0;
                let ptr3 = vec3.as_ptr() as i32;
                let len3 = vec3.len() as i32;
                *((base + 4) as *mut i32) = len3;
                *((base + 0) as *mut i32) = ptr3;
                let vec4 = t2_1;
                let ptr4 = vec4.as_ptr() as i32;
                let len4 = vec4.len() as i32;
                *((base + 12) as *mut i32) = len4;
                *((base + 8) as *mut i32) = ptr4;
            }
        }
        let vec9 = params0;
        let len9 = vec9.len() as i32;
        let layout9 = core::alloc::Layout::from_size_align_unchecked(vec9.len() * 16, 4);
        let result9 = if layout9.size() != 0 {
            let ptr = std::alloc::alloc(layout9);
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout9);
            }
            ptr
        } else {
            std::ptr::null_mut()
        };
        for (i, e) in vec9.into_iter().enumerate() {
            let base = result9 as i32 + (i as i32) * 16;
            {
                let (t6_0, t6_1) = e;
                let vec7 = t6_0;
                let ptr7 = vec7.as_ptr() as i32;
                let len7 = vec7.len() as i32;
                *((base + 4) as *mut i32) = len7;
                *((base + 0) as *mut i32) = ptr7;
                let vec8 = t6_1;
                let ptr8 = vec8.as_ptr() as i32;
                let len8 = vec8.len() as i32;
                *((base + 12) as *mut i32) = len8;
                *((base + 8) as *mut i32) = ptr8;
            }
        }
        let (result11_0, result11_1, result11_2) = match body0 {
            Some(e) => {
                let vec10 = e;
                let ptr10 = vec10.as_ptr() as i32;
                let len10 = vec10.len() as i32;

                (1i32, ptr10, len10)
            }
            None => {
                let e = ();
                {
                    let () = e;

                    (0i32, 0i32, 0i32)
                }
            }
        };
        let ptr12 = __HTTP_CLIENT_RET_AREA.0.as_mut_ptr() as i32;
        request(
            match method0 {
                Method::Get => 0,
                Method::Post => 1,
                Method::Put => 2,
                Method::Delete => 3,
                Method::Patch => 4,
                Method::Head => 5,
                Method::Options => 6,
            },
            ptr1,
            len1,
            result5 as i32,
            len5,
            result9 as i32,
            len9,
            result11_0,
            result11_1,
            result11_2,
            ptr12,
        );
        if layout5.size() != 0 {
            std::alloc::dealloc(result5, layout5);
        }
        if layout9.size() != 0 {
            std::alloc::dealloc(result9, layout9);
        }
        match i32::from(*((ptr12 + 0) as *const u8)) {
            0 => context.value_from_str(
                &serde_json::to_string(&Response {
                    status: i32::from(*((ptr12 + 4) as *const u16)) as u16,
                    headers: match i32::from(*((ptr12 + 8) as *const u8)) {
                        0 => None,
                        1 => Some({
                            let base15 = *((ptr12 + 12) as *const i32);
                            let len15 = *((ptr12 + 16) as *const i32);
                            let mut result15 = Vec::with_capacity(len15 as usize);
                            for i in 0..len15 {
                                let base = base15 + i * 16;
                                result15.push({
                                    let len13 = *((base + 4) as *const i32) as usize;
                                    let len14 = *((base + 12) as *const i32) as usize;

                                    (
                                        String::from_utf8(Vec::from_raw_parts(
                                            *((base + 0) as *const i32) as *mut _,
                                            len13,
                                            len13,
                                        ))
                                        .unwrap(),
                                        String::from_utf8(Vec::from_raw_parts(
                                            *((base + 8) as *const i32) as *mut _,
                                            len14,
                                            len14,
                                        ))
                                        .unwrap(),
                                    )
                                });
                            }
                            if len15 != 0 {
                                std::alloc::dealloc(
                                    base15 as *mut _,
                                    std::alloc::Layout::from_size_align_unchecked(
                                        (len15 as usize) * 16,
                                        4,
                                    ),
                                );
                            }

                            result15
                        }),
                        _ => panic!("invalid enum discriminant"),
                    },
                    body: match i32::from(*((ptr12 + 20) as *const u8)) {
                        0 => None,
                        1 => Some({
                            let len16 = *((ptr12 + 28) as *const i32) as usize;

                            String::from_utf8(Vec::from_raw_parts(
                                *((ptr12 + 24) as *const i32) as *mut _,
                                len16,
                                len16,
                            ))?
                        }),
                        _ => panic!("invalid enum discriminant"),
                    },
                })
                .unwrap(),
            ),
            1 => context.value_from_str(&HttpError(ptr12)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

#[allow(non_snake_case)]
fn HttpError(ptr12: i32) -> String {
    unsafe {
        match i32::from(*((ptr12 + 4) as *const u8)) {
            0 => {
                let len17 = *((ptr12 + 12) as *const i32) as usize;

                String::from_utf8(Vec::from_raw_parts(
                    *((ptr12 + 8) as *const i32) as *mut _,
                    len17,
                    len17,
                ))
                .unwrap()
            }
            1 => {
                let len18 = *((ptr12 + 12) as *const i32) as usize;

                String::from_utf8(Vec::from_raw_parts(
                    *((ptr12 + 8) as *const i32) as *mut _,
                    len18,
                    len18,
                ))
                .unwrap()
            }
            2 => {
                let len19 = *((ptr12 + 12) as *const i32) as usize;

                String::from_utf8(Vec::from_raw_parts(
                    *((ptr12 + 8) as *const i32) as *mut _,
                    len19,
                    len19,
                ))
                .unwrap()
            }
            3 => (i32::from(*((ptr12 + 8) as *const u16)) as u16).to_string(),
            4 => {
                let len20 = *((ptr12 + 12) as *const i32) as usize;

                String::from_utf8(Vec::from_raw_parts(
                    *((ptr12 + 8) as *const i32) as *mut _,
                    len20,
                    len20,
                ))
                .unwrap()
            }
            _ => panic!("invalid enum discriminant"),
        }
    }
}

#[repr(align(4))]
struct __HttpClientRetArea([u8; 32]);
static mut __HTTP_CLIENT_RET_AREA: __HttpClientRetArea = __HttpClientRetArea([0; 32]);

pub fn inject_http_client_dependency(context: &Context, global: &Value) -> anyhow::Result<()> {
    let http_client = context.object_value()?;
    http_client.set_property("request", context.wrap_callback(http_client_request)?)?;
    global.set_property("http_client", http_client)?;
    Ok(())
}
