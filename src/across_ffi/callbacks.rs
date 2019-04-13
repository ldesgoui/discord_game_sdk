use crate::{sys, to_result::ToResult, DiscordResult, Relationship};
use crossbeam_channel::Sender;
use std::ffi::{c_void, CStr};

pub(crate) extern "C" fn result(ptr: *mut c_void, res: sys::EDiscordResult) {
    prevent_unwind!();

    unsafe { Box::from_raw(ptr as *mut Sender<DiscordResult<()>>) }
        .try_send(res.to_result())
        .unwrap()
}

pub(crate) extern "C" fn result_string(
    ptr: *mut c_void,
    res: sys::EDiscordResult,
    cstr: *const i8,
) {
    prevent_unwind!();

    unsafe { Box::from_raw(ptr as *mut Sender<DiscordResult<String>>) }
        .try_send(res.to_result().map(|()| {
            unsafe { CStr::from_ptr(cstr) }
                .to_str()
                .unwrap()
                .to_string()
        }))
        .unwrap()
}

pub(crate) extern "C" fn result_bytes(
    ptr: *mut c_void,
    res: sys::EDiscordResult,
    buffer_ptr: *mut u8,
    len: u32,
) {
    prevent_unwind!();

    unsafe { Box::from_raw(ptr as *mut Sender<DiscordResult<Vec<u8>>>) }
        .try_send(
            res.to_result()
                .map(|()| unsafe { std::slice::from_raw_parts(buffer_ptr, len as usize) }.to_vec()),
        )
        .unwrap()
}

pub(crate) extern "C" fn result_from<S, E>(ptr: *mut c_void, res: sys::EDiscordResult, source: S)
where
    S: Into<E>,
{
    prevent_unwind!();

    unsafe { Box::from_raw(ptr as *mut Sender<DiscordResult<E>>) }
        .try_send(res.to_result().map(|()| source.into()))
        .unwrap()
}

pub(crate) extern "C" fn result_from_ptr<S, E>(
    ptr: *mut c_void,
    res: sys::EDiscordResult,
    source_ptr: *mut S,
) where
    S: Into<E> + Copy,
{
    prevent_unwind!();

    unsafe { Box::from_raw(ptr as *mut Sender<DiscordResult<E>>) }
        .try_send(res.to_result().map(|()| unsafe { *source_ptr }.into()))
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

    let callback: &mut F = unsafe { (callback_ptr as *mut F).as_mut() }.unwrap();

    callback(unsafe { *relationship_ptr }.into())
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

    log::log!(level, "SDK: {}", message);
}
