use quickjs_wasm_rs::{Context, Value};

#[link(wasm_import_module = "messaging")]
extern "C" {
    #[cfg_attr(target_arch = "wasm32", link_name = "pub::open")]
    fn p_open(_: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "pub::publish")]
    fn publish(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "sub::open")]
    fn s_open(_: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "sub::subscribe")]
    fn subscribe(_: i32, _: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "sub::receive")]
    fn receive(_: i32, _: i32, _: i32, _: i32);
}

/// creates a handle to a pub object
pub fn pub_open(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let vec0 = args[0].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __MESSAGING_RET_AREA.0.as_mut_ptr() as i32;
        p_open(ptr0, len0, ptr1);
        match i32::from(*(ptr1 as *const u8)) {
            0 => context.value_from_i32(*((ptr1 + 4) as *const i32)),
            1 => context.value_from_str(&MessagingError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

/// publish a message to a topic
pub fn pub_publish(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let vec0 = args[1].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let vec1 = args[2].as_str().unwrap().to_string();
        let ptr1 = vec1.as_ptr() as i32;
        let len1 = vec1.len() as i32;
        let ptr2 = __MESSAGING_RET_AREA.0.as_mut_ptr() as i32;
        publish(self0, ptr0, len0, ptr1, len1, ptr2);
        match i32::from(*(ptr2 as *const u8)) {
            0 => context.null_value(),
            1 => context.value_from_str(&MessagingError(ptr2)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

/// creates a handle to a sub object
pub fn sub_open(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let vec0 = args[0].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __MESSAGING_RET_AREA.0.as_mut_ptr() as i32;
        s_open(ptr0, len0, ptr1);
        match i32::from(*(ptr1 as *const u8)) {
            0 => context.value_from_i32(*((ptr1 + 4) as *const i32)),
            1 => context.value_from_str(&MessagingError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

/// subscribe to a topic
pub fn sub_subscribe(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let vec0 = args[1].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __MESSAGING_RET_AREA.0.as_mut_ptr() as i32;
        subscribe(self0, ptr0, len0, ptr1);
        match i32::from(*(ptr1 as *const u8)) {
            0 => {
                let len2 = *((ptr1 + 8) as *const i32) as usize;

                context.value_from_str(
                    &String::from_utf8(Vec::from_raw_parts(
                        *((ptr1 + 4) as *const i32) as *mut _,
                        len2,
                        len2,
                    ))
                    .unwrap(),
                )
            }
            1 => context.value_from_str(&MessagingError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

/// pull-based message delivery
pub fn sub_receive(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let vec0 = args[1].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __MESSAGING_RET_AREA.0.as_mut_ptr() as i32;
        receive(self0, ptr0, len0, ptr1);
        match i32::from(*(ptr1 as *const u8)) {
            0 => {
                let len2 = *((ptr1 + 8) as *const i32) as usize;

                let v = Vec::from_raw_parts(*((ptr1 + 4) as *const i32) as *mut _, len2, len2);

                context.array_buffer_value(&v)
            }
            1 => context.value_from_str(&MessagingError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

#[allow(non_snake_case)]
fn MessagingError(ptr1: i32) -> String {
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
struct __MessagingRetArea([u8; 16]);
static mut __MESSAGING_RET_AREA: __MessagingRetArea = __MessagingRetArea([0; 16]);

pub fn inject_messaging_dependency(context: &Context, global: &Value) -> anyhow::Result<()> {
    let _pub = context.object_value()?;
    _pub.set_property("open", context.wrap_callback(pub_open)?)?;
    _pub.set_property("publish", context.wrap_callback(pub_publish)?)?;

    let sub = context.object_value()?;
    sub.set_property("open", context.wrap_callback(sub_open)?)?;
    sub.set_property("subscribe", context.wrap_callback(sub_subscribe)?)?;
    sub.set_property("receive", context.wrap_callback(sub_receive)?)?;

    global.set_property("pub", _pub)?;
    global.set_property("sub", sub)?;

    Ok(())
}
