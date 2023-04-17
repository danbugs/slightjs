use quickjs_wasm_rs::{Context, Value};

#[link(wasm_import_module = "keyvalue")]
extern "C" {
    #[cfg_attr(target_arch = "wasm32", link_name = "keyvalue::open")]
    fn open(_: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "keyvalue::get")]
    fn get(_: i32, _: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "keyvalue::set")]
    fn set(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "keyvalue::keys")]
    fn keys(_: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "keyvalue::delete")]
    fn delete(_: i32, _: i32, _: i32, _: i32);
}

/// open a key-value store
fn keyvalue_open(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let vec0 = args[0].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __KEYVALUE_RET_AREA.0.as_mut_ptr() as i32;
        open(ptr0, len0, ptr1);
        match i32::from(*((ptr1 + 0) as *const u8)) {
            0 => context.value_from_i32(*((ptr1 + 4) as *const i32)),
            1 => context.value_from_str(&KeyvalueError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

/// get the payload for a given key
fn keyvalue_get(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let vec0 = args[1].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __KEYVALUE_RET_AREA.0.as_mut_ptr() as i32;
        get(self0, ptr0, len0, ptr1);
        match i32::from(*((ptr1 + 0) as *const u8)) {
            0 => {
                let len2 = *((ptr1 + 8) as *const i32) as usize;

                let v = Vec::from_raw_parts(*((ptr1 + 4) as *const i32) as *mut _, len2, len2);

                context.array_buffer_value(&v)
            },
            1 => context.value_from_str(&KeyvalueError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

/// set the payload for a given key
fn keyvalue_set(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let vec0 = args[1].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let vec1 = args[2].as_str().unwrap().to_string();
        let ptr1 = vec1.as_ptr() as i32;
        let len1 = vec1.len() as i32;
        let ptr2 = __KEYVALUE_RET_AREA.0.as_mut_ptr() as i32;
        set(self0, ptr0, len0, ptr1, len1, ptr2);
        match i32::from(*((ptr2 + 0) as *const u8)) {
            0 => context.null_value(),
            1 => context.value_from_str(&KeyvalueError(ptr2)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

/// list the keys in the store
fn keyvalue_keys(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let ptr0 = __KEYVALUE_RET_AREA.0.as_mut_ptr() as i32;
        keys(self0, ptr0);
        match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok({
                let base2 = *((ptr0 + 4) as *const i32);
                let len2 = *((ptr0 + 8) as *const i32);
                let mut result2 = Vec::with_capacity(len2 as usize);
                for i in 0..len2 {
                    let base = base2 + i * 8;
                    result2.push({
                        let len1 = *((base + 4) as *const i32) as usize;

                        String::from_utf8(Vec::from_raw_parts(
                            *((base + 0) as *const i32) as *mut _,
                            len1,
                            len1,
                        ))
                        .unwrap()
                    });
                }
                if len2 != 0 {
                    std::alloc::dealloc(
                        base2 as *mut _,
                        std::alloc::Layout::from_size_align_unchecked((len2 as usize) * 8, 4),
                    );
                }

                let arr = context.array_value().unwrap();
                for i in 0..len2 {
                    arr.append_property(
                        context
                            .value_from_str(&format!("{}", result2[i as usize]))
                            .unwrap(),
                    )
                    .unwrap();
                }

                arr
            }),
            1 => context.value_from_str(&KeyvalueError(ptr0)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

/// delete the payload for a given key
fn keyvalue_delete(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let vec0: String = args[1].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __KEYVALUE_RET_AREA.0.as_mut_ptr() as i32;

        delete(self0, ptr0, len0, ptr1);
        match i32::from(*((ptr1 + 0) as *const u8)) {
            0 => context.null_value(),
            1 => context.value_from_str(&KeyvalueError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}
#[allow(non_snake_case)]
fn KeyvalueError(ptr1: i32) -> String {
    unsafe {
        match i32::from(*((ptr1 + 4) as *const u8)) {
            0 => {
                let len2 = *((ptr1 + 12) as *const i32) as usize;

                String::from_utf8(Vec::from_raw_parts(
                    *((ptr1 + 8) as *const i32) as *mut _,
                    len2,
                    len2,
                ))
                .unwrap()
            }
            1 => {
                let len3 = *((ptr1 + 12) as *const i32) as usize;

                String::from_utf8(Vec::from_raw_parts(
                    *((ptr1 + 8) as *const i32) as *mut _,
                    len3,
                    len3,
                ))
                .unwrap()
            }
            2 => {
                let len4 = *((ptr1 + 12) as *const i32) as usize;

                String::from_utf8(Vec::from_raw_parts(
                    *((ptr1 + 8) as *const i32) as *mut _,
                    len4,
                    len4,
                ))
                .unwrap()
            }
            3 => {
                let len5 = *((ptr1 + 12) as *const i32) as usize;

                String::from_utf8(Vec::from_raw_parts(
                    *((ptr1 + 8) as *const i32) as *mut _,
                    len5,
                    len5,
                ))
                .unwrap()
            }
            4 => {
                let len6 = *((ptr1 + 12) as *const i32) as usize;

                String::from_utf8(Vec::from_raw_parts(
                    *((ptr1 + 8) as *const i32) as *mut _,
                    len6,
                    len6,
                ))
                .unwrap()
            }
            5 => {
                let len7 = *((ptr1 + 12) as *const i32) as usize;

                String::from_utf8(Vec::from_raw_parts(
                    *((ptr1 + 8) as *const i32) as *mut _,
                    len7,
                    len7,
                ))
                .unwrap()
            }
            6 => {
                let len8 = *((ptr1 + 12) as *const i32) as usize;

                String::from_utf8(Vec::from_raw_parts(
                    *((ptr1 + 8) as *const i32) as *mut _,
                    len8,
                    len8,
                ))
                .unwrap()
            }
            7 => {
                let len9 = *((ptr1 + 12) as *const i32) as usize;

                String::from_utf8(Vec::from_raw_parts(
                    *((ptr1 + 8) as *const i32) as *mut _,
                    len9,
                    len9,
                ))
                .unwrap()
            }
            _ => panic!("invalid enum discriminant"),
        }
    }
}

#[repr(align(4))]
struct __KeyvalueRetArea([u8; 16]);
static mut __KEYVALUE_RET_AREA: __KeyvalueRetArea = __KeyvalueRetArea([0; 16]);

pub fn inject_keyvalue_dependency(context: &Context, global: &Value) -> anyhow::Result<()> {
    let keyvalue = context.object_value()?;
    keyvalue.set_property("open", context.wrap_callback(keyvalue_open)?)?;
    keyvalue.set_property("get", context.wrap_callback(keyvalue_get)?)?;
    keyvalue.set_property("set", context.wrap_callback(keyvalue_set)?)?;
    keyvalue.set_property("keys", context.wrap_callback(keyvalue_keys)?)?;
    keyvalue.set_property("delete", context.wrap_callback(keyvalue_delete)?)?;
    global.set_property("keyvalue", keyvalue)?;
    Ok(())
}
