use quickjs_wasm_rs::{Context, Value};

#[link(wasm_import_module = "configs")]
extern "C" {
    #[cfg_attr(target_arch = "wasm32", link_name = "configs::open")]
    fn c_open(_: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "configs::get")]
    fn c_get(_: i32, _: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "configs::set")]
    fn c_set(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
}

/// creates a handle to a configs object
pub fn configs_open(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let vec0 = args[0].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __CONFIGS_RET_AREA.0.as_mut_ptr() as i32;
        c_open(ptr0, len0, ptr1);
        match i32::from(*(ptr1 as *const u8)) {
            0 => context.value_from_i32(*((ptr1 + 4) as *const i32)),
            1 => context.value_from_str(&ConfigsError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

/// get a config value through a key
pub fn configs_get(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0 = args[0].as_i32_unchecked();
        let vec0 = args[1].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __CONFIGS_RET_AREA.0.as_mut_ptr() as i32;
        c_get(self0, ptr0, len0, ptr1);
        match i32::from(*(ptr1 as *const u8)) {
            0 => context.array_buffer_value({
                let len2 = *((ptr1 + 8) as *const i32) as usize;

                &Vec::from_raw_parts(*((ptr1 + 4) as *const i32) as *mut _, len2, len2)
            }),
            1 => context.value_from_str(&ConfigsError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

/// set a config key and value
pub fn configs_set(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0 = args[0].as_i32_unchecked();
        let vec0 = args[1].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let vec1 = args[2].as_str().unwrap().to_string();
        let ptr1 = vec1.as_ptr() as i32;
        let len1 = vec1.len() as i32;
        let ptr2 = __CONFIGS_RET_AREA.0.as_mut_ptr() as i32;
        c_set(self0, ptr0, len0, ptr1, len1, ptr2);
        match i32::from(*(ptr2 as *const u8)) {
            0 => context.null_value(),
            1 => context.value_from_str(&ConfigsError(ptr2)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

#[repr(align(4))]
struct __ConfigsRetArea([u8; 16]);
static mut __CONFIGS_RET_AREA: __ConfigsRetArea = __ConfigsRetArea([0; 16]);

#[allow(non_snake_case)]
fn ConfigsError(ptr1: i32) -> String {
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
            _ => panic!("invalid enum discriminant"),
        }
    }
}

pub fn inject_configs_dependency(context: &Context, global: &Value) -> anyhow::Result<()> {
    let configs = context.object_value()?;
    configs.set_property("open", context.wrap_callback(configs_open)?)?;
    configs.set_property("get", context.wrap_callback(configs_get)?)?;
    configs.set_property("set", context.wrap_callback(configs_set)?)?;
    global.set_property("configs", configs)?;
    Ok(())
}
