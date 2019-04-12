use crate::prelude::*;

pub(crate) extern "C" fn result(ptr: *mut c_void, res: sys::EDiscordResult) {
    prevent_unwind!();

    unsafe { Box::from_raw(ptr as *mut Sender<Result<()>>) }
        .try_send(res.to_result())
        .unwrap()
}

pub(crate) extern "C" fn result_string(
    ptr: *mut c_void,
    res: sys::EDiscordResult,
    cstr: *const i8,
) {
    prevent_unwind!();

    unsafe { Box::from_raw(ptr as *mut Sender<Result<String>>) }
        .try_send(res.to_result().map(|()| unsafe { string_from_cstr(cstr) }))
        .unwrap()
}

pub(crate) extern "C" fn result_bytes(
    ptr: *mut c_void,
    res: sys::EDiscordResult,
    buffer_ptr: *mut u8,
    len: u32,
) {
    prevent_unwind!();

    unsafe { Box::from_raw(ptr as *mut Sender<Result<Vec<u8>>>) }
        .try_send(
            res.to_result()
                .map(|()| unsafe { std::slice::from_raw_parts(buffer_ptr, len as usize) }.to_vec()),
        )
        .unwrap()
}

pub(crate) extern "C" fn result_from_sys<S>(
    ptr: *mut c_void,
    res: sys::EDiscordResult,
    source: S::Source,
) where
    S: FromSys,
{
    prevent_unwind!();

    unsafe { Box::from_raw(ptr as *mut Sender<Result<S>>) }
        .try_send(res.to_result().map(|()| S::from_sys(&source)))
        .unwrap()
}

pub(crate) extern "C" fn result_from_sys_ptr<S>(
    ptr: *mut c_void,
    res: sys::EDiscordResult,
    source_ptr: *mut S::Source,
) where
    S: FromSys,
{
    prevent_unwind!();

    unsafe { Box::from_raw(ptr as *mut Sender<Result<S>>) }
        .try_send(
            res.to_result()
                .map(|()| unsafe { S::from_sys_ptr(source_ptr) }),
        )
        .unwrap()
}

pub(crate) extern "C" fn filter_relationship<F>(
    callback_ptr: *mut c_void,
    relationship_ptr: *mut sys::DiscordRelationship,
) -> bool
where
    F: FnMut(Relationship) -> bool,
{
    prevent_unwind!();

    let mut callback: &mut F = unsafe { (callback_ptr as *mut F).as_mut() }.unwrap();

    callback(unsafe { Relationship::from_sys_ptr(relationship_ptr) })
}

pub(crate) extern "C" fn log(_: *mut c_void, level: sys::EDiscordLogLevel, message: *const i8) {
    prevent_unwind!();

    let level = match level {
        sys::DiscordLogLevel_Error => log::Level::Error,
        sys::DiscordLogLevel_Warn => log::Level::Warn,
        sys::DiscordLogLevel_Info => log::Level::Info,
        sys::DiscordLogLevel_Debug => log::Level::Debug,
        _ => panic!("enum"),
    };

    let message = unsafe { std::ffi::CStr::from_ptr(message) }
        .to_str()
        .unwrap();

    log!(level, "SDK: {}", message);
}
