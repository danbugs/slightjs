use quickjs_wasm_rs::{Context, Value};

#[link(wasm_import_module = "sql")]
extern "C" {
    #[cfg_attr(target_arch = "wasm32", link_name = "sql::open")]
    fn sq_open(_: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "sql::query")]
    fn query(_: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "sql::exec")]
    fn exec(_: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "statement::prepare")]
    fn prepare(_: i32, _: i32, _: i32, _: i32) -> i32;
}

/// open a sql client
pub fn sql_open(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let vec0 = args[0].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __SQL_RET_AREA.0.as_mut_ptr() as i32;
        sq_open(ptr0, len0, ptr1);
        match i32::from(*((ptr1 + 0) as *const u8)) {
            0 => context.value_from_i32(*((ptr1 + 4) as *const i32)),
            1 => context.value_from_str(&SqlError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

pub fn sql_query(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let q0: i32 = args[1].as_i32_unchecked();
        let ptr0 = __SQL_RET_AREA.0.as_mut_ptr() as i32;
        query(self0, q0, ptr0);
        match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok({
                let base7 = *((ptr0 + 4) as *const i32);
                let len7 = *((ptr0 + 8) as *const i32);
                let result7 = context.array_value().unwrap();
                for i in 0..len7 {
                    let base = base7 + i * 24;
                    result7.append_property({
                        let len1 = *((base + 4) as *const i32) as usize;

                        let row_item = context.object_value()?;
                        row_item.set_property(
                            "field_name",
                            context.value_from_str(
                                &String::from_utf8(Vec::from_raw_parts(
                                    *((base + 0) as *const i32) as *mut _,
                                    len1,
                                    len1,
                                ))
                                .unwrap(),
                            )?,
                        )?;

                        row_item.set_property(
                            "type",
                            context.value_from_str(&match i32::from(*((base + 8) as *const u8)) {
                                0 => "Int32",
                                1 => "Int64",
                                2 => "Uint32",
                                3 => "Uint64",
                                4 => "Float",
                                5 => "Double",
                                6 => "Str",
                                7 => "Boolean",
                                8 => "Date",
                                9 => "Time",
                                10 => "Timestamp",
                                11 => "Binary",
                                12 => "Null",
                                _ => panic!("invalid enum discriminant"),
                            })?,
                        )?;

                        row_item.set_property(
                            "value",
                            match i32::from(*((base + 8) as *const u8)) {
                                0 => context.value_from_i32(*((base + 16) as *const i32))?,
                                1 => context.value_from_i64(*((base + 16) as *const i64))?,
                                2 => context.value_from_i32(*((base + 16) as *const i32))?,
                                3 => context.value_from_i64(*((base + 16) as *const i64))?,
                                4 => context.value_from_f64(*((base + 16) as *const f64))?,
                                5 => context.value_from_f64(*((base + 16) as *const f64))?,
                                6 => context.value_from_str(
                                    &String::from_utf8(Vec::from_raw_parts(
                                        *((base + 16) as *const i32) as *mut _,
                                        *((base + 20) as *const i32) as usize,
                                        *((base + 20) as *const i32) as usize,
                                    ))
                                    .unwrap(),
                                )?,
                                7 => context.value_from_bool(
                                    match i32::from(*((base + 16) as *const u8)) {
                                        0 => false,
                                        1 => true,
                                        _ => panic!("invalid bool discriminant"),
                                    },
                                )?,
                                8 => context.value_from_str(
                                    &String::from_utf8(Vec::from_raw_parts(
                                        *((base + 16) as *const i32) as *mut _,
                                        *((base + 20) as *const i32) as usize,
                                        *((base + 20) as *const i32) as usize,
                                    ))
                                    .unwrap(),
                                )?,
                                9 => context.value_from_str(
                                    &String::from_utf8(Vec::from_raw_parts(
                                        *((base + 16) as *const i32) as *mut _,
                                        *((base + 20) as *const i32) as usize,
                                        *((base + 20) as *const i32) as usize,
                                    ))
                                    .unwrap(),
                                )?,
                                10 => context.value_from_str(
                                    &String::from_utf8(Vec::from_raw_parts(
                                        *((base + 16) as *const i32) as *mut _,
                                        *((base + 20) as *const i32) as usize,
                                        *((base + 20) as *const i32) as usize,
                                    ))
                                    .unwrap(),
                                )?,
                                11 => context.value_from_str(
                                    &String::from_utf8(Vec::from_raw_parts(
                                        *((base + 16) as *const i32) as *mut _,
                                        *((base + 20) as *const i32) as usize,
                                        *((base + 20) as *const i32) as usize,
                                    ))
                                    .unwrap(),
                                )?,
                                12 => context.null_value()?,
                                _ => panic!("invalid enum discriminant"),
                            },
                        )?;

                        row_item
                    })?;
                }
                if len7 != 0 {
                    std::alloc::dealloc(
                        base7 as *mut _,
                        std::alloc::Layout::from_size_align_unchecked((len7 as usize) * 24, 8),
                    );
                }

                result7
            }),
            1 => context.value_from_str(&SqlError(ptr0)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

pub fn sql_exec(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let q0: i32 = args[1].as_i32_unchecked();
        let ptr0 = __SQL_RET_AREA.0.as_mut_ptr() as i32;
        exec(self0, q0, ptr0);
        match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => context.null_value(),
            1 => context.value_from_str(&SqlError(ptr0)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

pub fn statement_prepare(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let vec0 = args[0].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let vec2_r = {
            let js_arr = args[1].clone();
            let js_arr_len = js_arr.get_property("length")?.as_i32_unchecked();
            let mut result = Vec::with_capacity(js_arr_len as usize);
            for i in 0..js_arr_len {
                result.push(js_arr.get_indexed_property(i.try_into().unwrap())?.as_str()?.to_string());
            }
            result
        };
        let vec2: &[&str] = &vec2_r.iter().map(|s| s.as_str()).collect::<Vec<_>>();
        let len2 = vec2.len() as i32;
        let layout2 = core::alloc::Layout::from_size_align_unchecked(vec2.len() * 8, 4);
        let result2 = if layout2.size() != 0
        {
          let ptr = std::alloc::alloc(layout2);
          if ptr.is_null()
          {
            std::alloc::handle_alloc_error(layout2);
          }
          ptr
        }else {
          std::ptr::null_mut()
        };
        for (i, e) in vec2.into_iter().enumerate() {
          let base = result2 as i32 + (i as i32) * 8;
          {
            let vec1 = e;
            let ptr1 = vec1.as_ptr() as i32;
            let len1 = vec1.len() as i32;
            *((base + 4) as *mut i32) = len1;
            *((base + 0) as *mut i32) = ptr1;
            
          }}
        let ret = prepare(ptr0, len0, result2 as i32, len2);
        context.value_from_i32(ret)
    }
}

#[repr(align(4))]
struct __SqlRetArea([u8; 16]);
static mut __SQL_RET_AREA: __SqlRetArea = __SqlRetArea([0; 16]);

#[allow(non_snake_case)]
fn SqlError(ptr1: i32) -> String {
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
            _ => panic!("invalid enum discriminant"),
        }
    }
}

pub fn inject_sql_dependency(context: &Context, global: &Value) -> anyhow::Result<()> {
    let sql = context.object_value()?;
    sql.set_property("open", context.wrap_callback(sql_open)?)?;
    sql.set_property("query", context.wrap_callback(sql_query)?)?;
    sql.set_property("exec", context.wrap_callback(sql_exec)?)?;
    global.set_property("sql", sql)?;

    let statement = context.object_value()?;
    statement.set_property("prepare", context.wrap_callback(statement_prepare)?)?;
    global.set_property("statement", statement)?;

    Ok(())
}