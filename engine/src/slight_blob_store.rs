use quickjs_wasm_rs::{Context, Value};

#[link(wasm_import_module = "blob-store")]
extern "C" {
    #[cfg_attr(target_arch = "wasm32", link_name = "container::open")]
    fn bs_open(_: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "container::name")]
    fn name(_: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "container::info")]
    fn info(_: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "container::read-object")]
    fn read_object(_: i32, _: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "container::write-object")]
    fn write_object(_: i32, _: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "container::list-objects")]
    fn list_objects(_: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "container::delete-object")]
    fn delete_object(_: i32, _: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "container::delete-objects")]
    fn delete_objects(_: i32, _: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "container::has-object")]
    fn has_object(_: i32, _: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "container::object-info")]
    fn object_info(_: i32, _: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "container::clear")]
    fn clear(_: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "write-stream::write")]
    fn write(_: i32, _: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "write-stream::close")]
    fn close(_: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "read-stream::read")]
    fn read(_: i32, _: i64, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "read-stream::available")]
    fn available(_: i32, _: i32);
}

pub fn container_open(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let vec0 = args[0].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __BLOB_STORE_RET_AREA.0.as_mut_ptr() as i32;
        bs_open(ptr0, len0, ptr1);
        match i32::from(*((ptr1 + 0) as *const u8)) {
            0 => context.value_from_i32(*((ptr1 + 4) as *const i32)),
            1 => context.value_from_str(&BlobStoreError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

pub fn container_name(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let ptr0 = __BLOB_STORE_RET_AREA.0.as_mut_ptr() as i32;
        name(self0, ptr0);
        match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => context.value_from_str({
                let len1 = *((ptr0 + 8) as *const i32) as usize;

                &String::from_utf8(Vec::from_raw_parts(
                    *((ptr0 + 4) as *const i32) as *mut _,
                    len1,
                    len1,
                ))
                .unwrap()
            }),
            1 => context.value_from_str(&BlobStoreError(ptr0)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

pub fn container_info(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let ptr0 = __BLOB_STORE_RET_AREA.0.as_mut_ptr() as i32;
        info(self0, ptr0);
        match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => {
                let len1 = *((ptr0 + 12) as *const i32) as usize;

                let obj = context.object_value()?;
                obj.set_property(
                    "name",
                    context.value_from_str(
                        &String::from_utf8(Vec::from_raw_parts(
                            *((ptr0 + 4) as *const i32) as *mut _,
                            len1,
                            len1,
                        ))
                        .unwrap(),
                    )?,
                )?;
                obj.set_property(
                    "created_at",
                    context.value_from_i64(*((ptr0 + 8) as *const i64))?,
                )?;
                Ok(obj)
            }
            1 => context.value_from_str(&BlobStoreError(ptr0)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

pub fn container_read_object(
    context: &Context,
    _this: &Value,
    args: &[Value],
) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let vec0 = args[1].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __BLOB_STORE_RET_AREA.0.as_mut_ptr() as i32;
        read_object(self0, ptr0, len0, ptr1);
        match i32::from(*((ptr1 + 0) as *const u8)) {
            0 => context.value_from_i32(*((ptr1 + 4) as *const i32)),
            1 => context.value_from_str(&BlobStoreError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

pub fn container_write_object(
    context: &Context,
    _this: &Value,
    args: &[Value],
) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let vec0 = args[1].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __BLOB_STORE_RET_AREA.0.as_mut_ptr() as i32;
        write_object(self0, ptr0, len0, ptr1);
        match i32::from(*((ptr1 + 0) as *const u8)) {
            0 => context.value_from_i32(*((ptr1 + 4) as *const i32)),
            1 => context.value_from_str(&BlobStoreError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

pub fn container_list_objects(
    context: &Context,
    _this: &Value,
    args: &[Value],
) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let ptr0 = __BLOB_STORE_RET_AREA.0.as_mut_ptr() as i32;
        list_objects(self0, ptr0);
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
            1 => context.value_from_str(&BlobStoreError(ptr0)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

pub fn container_delete_object(
    context: &Context,
    _this: &Value,
    args: &[Value],
) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let vec0 = args[1].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __BLOB_STORE_RET_AREA.0.as_mut_ptr() as i32;
        delete_object(self0, ptr0, len0, ptr1);
        match i32::from(*((ptr1 + 0) as *const u8)) {
            0 => context.null_value(),
            1 => context.value_from_str(&BlobStoreError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

pub fn container_delete_objects(
    context: &Context,
    _this: &Value,
    args: &[Value],
) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let vec1 = {
            let js_arr = args[1].clone();
            let js_arr_len = js_arr.get_property("length")?.as_i32_unchecked();
            let mut result = Vec::with_capacity(js_arr_len as usize);
            for i in 0..js_arr_len {
                result.push(
                    js_arr
                        .get_indexed_property(i.try_into().unwrap())?
                        .as_str()?
                        .to_string(),
                );
            }
            result
        };
        let len1 = vec1.len() as i32;
        let layout1 = core::alloc::Layout::from_size_align_unchecked(vec1.len() * 8, 4);
        let result1 = if layout1.size() != 0 {
            let ptr = std::alloc::alloc(layout1);
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout1);
            }
            ptr
        } else {
            std::ptr::null_mut()
        };
        for (i, e) in vec1.into_iter().enumerate() {
            let base = result1 as i32 + (i as i32) * 8;
            {
                let vec0 = e;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                *((base + 4) as *mut i32) = len0;
                *((base + 0) as *mut i32) = ptr0;
            }
        }
        let ptr2 = __BLOB_STORE_RET_AREA.0.as_mut_ptr() as i32;
        delete_objects(self0, result1 as i32, len1, ptr2);
        if layout1.size() != 0 {
            std::alloc::dealloc(result1, layout1);
        }
        match i32::from(*((ptr2 + 0) as *const u8)) {
            0 => context.null_value(),
            1 => context.value_from_str(&BlobStoreError(ptr2)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

pub fn container_has_object(
    context: &Context,
    _this: &Value,
    args: &[Value],
) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let vec0 = args[1].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __BLOB_STORE_RET_AREA.0.as_mut_ptr() as i32;
        has_object(self0, ptr0, len0, ptr1);
        match i32::from(*((ptr1 + 0) as *const u8)) {
            0 => context.value_from_bool(match i32::from(*((ptr1 + 4) as *const u8)) {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }),
            1 => context.value_from_str(&BlobStoreError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

pub fn container_object_info(
    context: &Context,
    _this: &Value,
    args: &[Value],
) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let vec0 = args[1].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __BLOB_STORE_RET_AREA.0.as_mut_ptr() as i32;
        object_info(self0, ptr0, len0, ptr1);
        match i32::from(*((ptr1 + 0) as *const u8)) {
            0 => Ok({
                let len2 = *((ptr1 + 12) as *const i32) as usize;
                let len3 = *((ptr1 + 20) as *const i32) as usize;

                let obj = context.object_value()?;

                obj.set_property(
                    "name",
                    context.value_from_str(&String::from_utf8(Vec::from_raw_parts(
                        *((ptr1 + 8) as *const i32) as *mut _,
                        len2,
                        len2,
                    ))?)?,
                )?;

                obj.set_property(
                    "container",
                    context.value_from_str(&String::from_utf8(Vec::from_raw_parts(
                        *((ptr1 + 16) as *const i32) as *mut _,
                        len3,
                        len3,
                    ))?)?,
                )?;

                obj.set_property(
                    "created_at",
                    context.value_from_f64(*((ptr1 + 24) as *const i64) as f64)?,
                )?;

                obj.set_property(
                    "size",
                    context.value_from_f64(*((ptr1 + 32) as *const i64) as f64)?,
                )?;

                obj
            }),
            1 => context.value_from_str(&BlobStoreError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

pub fn container_clear(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let ptr0 = __BLOB_STORE_RET_AREA.0.as_mut_ptr() as i32;
        clear(self0, ptr0);
        match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => context.null_value(),
            1 => context.value_from_str(&BlobStoreError(ptr0)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

pub fn write_stream_write(
    context: &Context,
    _this: &Value,
    args: &[Value],
) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let vec0 = args[1].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __BLOB_STORE_RET_AREA.0.as_mut_ptr() as i32;
        write(self0, ptr0, len0, ptr1);
        match i32::from(*((ptr1 + 0) as *const u8)) {
            0 => context.null_value(),
            1 => context.value_from_str(&BlobStoreError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

pub fn write_stream_close(
    context: &Context,
    _this: &Value,
    args: &[Value],
) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let ptr0 = __BLOB_STORE_RET_AREA.0.as_mut_ptr() as i32;
        close(self0, ptr0);
        match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => context.null_value(),
            1 => context.value_from_str(&BlobStoreError(ptr0)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

pub fn read_stream_read(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let size = args[1].as_f64_unchecked() as i64;
        let ptr0 = __BLOB_STORE_RET_AREA.0.as_mut_ptr() as i32;
        read(self0, size, ptr0);
        match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(match i32::from(*((ptr0 + 4) as *const u8)) {
                0 => context.null_value()?,
                1 => context.array_buffer_value(&{
                    let len1 = *((ptr0 + 12) as *const i32) as usize;

                    Vec::from_raw_parts(*((ptr0 + 8) as *const i32) as *mut _, len1, len1)
                })?,
                _ => panic!("invalid enum discriminant"),
            }),
            1 => context.value_from_str(&BlobStoreError(ptr0)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

pub fn read_stream_available(
    context: &Context,
    _this: &Value,
    args: &[Value],
) -> anyhow::Result<Value> {
    unsafe {
        let self0: i32 = args[0].as_i32_unchecked();
        let ptr0 = __BLOB_STORE_RET_AREA.0.as_mut_ptr() as i32;
        available(self0, ptr0);
        match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => context.value_from_u64(*((ptr0 + 8) as *const i64) as u64),
            1 => context.value_from_str(&BlobStoreError(ptr0)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

#[allow(non_snake_case)]
fn BlobStoreError(ptr1: i32) -> String {
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
            _ => panic!("invalid enum discriminant"),
        }
    }
}

#[repr(align(8))]
struct __BlobStoreRetArea([u8; 40]);
static mut __BLOB_STORE_RET_AREA: __BlobStoreRetArea = __BlobStoreRetArea([0; 40]);

pub fn inject_blob_store_dependency(context: &Context, global: &Value) -> anyhow::Result<()> {
    let container = context.object_value()?;
    container.set_property("open", context.wrap_callback(container_open)?)?;
    container.set_property("name", context.wrap_callback(container_name)?)?;
    container.set_property("info", context.wrap_callback(container_info)?)?;
    container.set_property("read_object", context.wrap_callback(container_read_object)?)?;
    container.set_property("write_object", context.wrap_callback(container_write_object)?)?;
    container.set_property("list_objects", context.wrap_callback(container_list_objects)?)?;
    container.set_property("delete_object", context.wrap_callback(container_delete_object)?)?;
    container.set_property("delete_objects", context.wrap_callback(container_delete_objects)?)?;
    container.set_property("has_object", context.wrap_callback(container_has_object)?)?;
    container.set_property("object_info", context.wrap_callback(container_object_info)?)?;
    container.set_property("clear", context.wrap_callback(container_clear)?)?;

    let write_stream = context.object_value()?;
    write_stream.set_property("write", context.wrap_callback(write_stream_write)?)?;
    write_stream.set_property("close", context.wrap_callback(write_stream_close)?)?;

    let read_stream = context.object_value()?;
    read_stream.set_property("read", context.wrap_callback(read_stream_read)?)?;
    read_stream.set_property("available", context.wrap_callback(read_stream_available)?)?;

    global.set_property("container", container)?;
    global.set_property("write_stream", write_stream)?;
    global.set_property("read_stream", read_stream)?;
    Ok(())
}