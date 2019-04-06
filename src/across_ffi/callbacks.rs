use crate::prelude::*;

pub(crate) extern "C" fn result<F>(callback_ptr: *mut c_void, res: sys::EDiscordResult)
where
    F: FnMut(Result<()>),
{
    let mut callback: Box<F> = unsafe { Box::from_raw(callback_ptr as *mut F) };

    callback(res.to_result())
}

pub(crate) extern "C" fn result_from_sys<F, S>(
    callback_ptr: *mut c_void,
    res: sys::EDiscordResult,
    source_ptr: *mut S::Source,
) where
    F: FnMut(Result<S>),
    S: FromSys,
{
    let mut callback: Box<F> = unsafe { Box::from_raw(callback_ptr as *mut F) };

    callback(
        res.to_result()
            .map(|()| unsafe { S::from_sys_ptr(source_ptr) }),
    )
}

pub(crate) extern "C" fn slice<F>(
    callback_ptr: *mut c_void,
    res: sys::EDiscordResult,
    buffer_ptr: *mut u8,
    len: u32,
) where
    F: FnMut(Result<&[u8]>) + Sized,
{
    let mut callback: Box<F> = unsafe { Box::from_raw(callback_ptr as *mut F) };

    callback(
        res.to_result()
            .map(|()| unsafe { std::slice::from_raw_parts(buffer_ptr, len as usize) }),
    )
}

pub(crate) extern "C" fn filter_relationship<F>(
    callback_ptr: *mut c_void,
    relationship_ptr: *mut sys::DiscordRelationship,
) -> bool
where
    F: FnMut(Relationship) -> bool,
{
    let mut callback: Box<F> = unsafe { Box::from_raw(callback_ptr as *mut F) };

    callback(unsafe { Relationship::from_sys_ptr(relationship_ptr) })
}

pub(crate) extern "C" fn log(_: *mut c_void, level: sys::EDiscordLogLevel, message: *const i8) {
    if message.is_null() {
        panic!("log_hook was passed a null pointer");
    }

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

    log::log!(target: "discord_game_sdk", level, "[SDK] {}", message);
}
