use quickjs_wasm_rs::{Context, Value};

#[link(wasm_import_module = "distributed-locking")]
extern "C" {
    #[cfg_attr(target_arch = "wasm32", link_name = "distributed-locking::open")]
    fn dl_open(_: i32, _: i32, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "distributed-locking::lock")]
    fn lock(_: i32, _: i32, _: i32, _: i32);
    #[cfg_attr(
        target_arch = "wasm32",
        link_name = "distributed-locking::lock-with-time-to-live"
    )]
    fn lock_with_time_to_live(_: i32, _: i32, _: i32, _: i64, _: i32);
    #[cfg_attr(target_arch = "wasm32", link_name = "distributed-locking::unlock")]
    fn unlock(_: i32, _: i32, _: i32, _: i32);
}

/// open a distributed-locking object
pub fn distributed_locking_open(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let vec0 = args[0].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __DISTRIBUTED_LOCKING_RET_AREA.0.as_mut_ptr() as i32;
        dl_open(ptr0, len0, ptr1);
        match i32::from(*((ptr1 + 0) as *const u8)) {
            0 => context.value_from_i32(*((ptr1 + 4) as *const i32)),
            1 => context.value_from_str(&DistributedLockingError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

/// creates a lock with a name, returns the lock key
pub fn distributed_locking_lock(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0 = args[0].as_i32_unchecked();
        let vec0 = args[1].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __DISTRIBUTED_LOCKING_RET_AREA.0.as_mut_ptr() as i32;
        lock(self0, ptr0, len0, ptr1);
        match i32::from(*((ptr1 + 0) as *const u8)) {
            0 => context.array_buffer_value({
                let len2 = *((ptr1 + 8) as *const i32) as usize;

                &Vec::from_raw_parts(*((ptr1 + 4) as *const i32) as *mut _, len2, len2)
            }),
            1 => context.value_from_str(&DistributedLockingError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

/// creates a lock with a lease id, hence giving the lock a TTL
pub fn distributed_locking_lock_with_time_to_live(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0 = args[0].as_i32_unchecked();
        let vec0 = args[1].as_str().unwrap().to_string();
        let time_to_live_in_secs = args[2].as_f64().unwrap() as i64;
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __DISTRIBUTED_LOCKING_RET_AREA.0.as_mut_ptr() as i32;
        lock_with_time_to_live(
            self0,
            ptr0,
            len0,
            time_to_live_in_secs,
            ptr1,
        );
        match i32::from(*((ptr1 + 0) as *const u8)) {
            0 => context.array_buffer_value({
                let len2 = *((ptr1 + 8) as *const i32) as usize;

                &Vec::from_raw_parts(*((ptr1 + 4) as *const i32) as *mut _, len2, len2)
            }),
            1 => context.value_from_str(&DistributedLockingError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

/// unlock a lock given a lock key
pub fn distributed_locking_unlock(context: &Context, _this: &Value, args: &[Value]) -> anyhow::Result<Value> {
    unsafe {
        let self0 = args[0].as_i32_unchecked();
        let vec0 = args[1].as_str().unwrap().to_string();
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = __DISTRIBUTED_LOCKING_RET_AREA.0.as_mut_ptr() as i32;
        unlock(self0, ptr0, len0, ptr1);
        match i32::from(*((ptr1 + 0) as *const u8)) {
            0 => context.null_value(),
            1 => context.value_from_str(&DistributedLockingError(ptr1)),
            _ => panic!("invalid enum discriminant"),
        }
    }
}

#[repr(align(4))]
struct __DistributedLockingRetArea([u8; 16]);
static mut __DISTRIBUTED_LOCKING_RET_AREA: __DistributedLockingRetArea =
    __DistributedLockingRetArea([0; 16]);

#[allow(non_snake_case)]
fn DistributedLockingError(ptr1: i32) -> String {
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
            _ => panic!("invalid enum discriminant"),
        }
    }
}

pub fn inject_distributed_locking_dependency(
    context: &Context,
    global: &Value,
) -> anyhow::Result<()> {
    let distributed_locking = context.object_value()?;
    distributed_locking.set_property("open", context.wrap_callback(distributed_locking_open)?)?;
    distributed_locking.set_property("lock", context.wrap_callback(distributed_locking_lock)?)?;
    distributed_locking.set_property("lock_with_time_to_live", context.wrap_callback(distributed_locking_lock_with_time_to_live)?)?;
    distributed_locking.set_property("unlock", context.wrap_callback(distributed_locking_unlock)?)?;
    global.set_property("distributed_locking", distributed_locking)?;
    Ok(())
}
