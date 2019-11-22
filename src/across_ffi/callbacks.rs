use crate::{
    panic_messages::{NOT_UTF8, SEND_FAIL},
    sys,
    to_result::ToResult,
    Relationship, Result,
};
use crossbeam_channel::Sender;
use std::ffi::{c_void, CStr};

pub(crate) extern "C" fn result(ptr: *mut c_void, res: sys::EDiscordResult) {
    prevent_unwind!();

    unsafe { Box::from_raw(ptr as *mut Sender<Result<()>>) }
        .try_send(res.to_result())
        .expect(SEND_FAIL)
}

pub(crate) extern "C" fn result_string(
    ptr: *mut c_void,
    res: sys::EDiscordResult,
    cstr: *const i8,
) {
    prevent_unwind!();

    unsafe { Box::from_raw(ptr as *mut Sender<Result<String>>) }
        .try_send(res.to_result().map(|()| {
            unsafe { CStr::from_ptr(cstr) }
                .to_str()
                .expect(NOT_UTF8)
                .to_string()
        }))
        .expect(SEND_FAIL)
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
        .expect(SEND_FAIL)
}

pub(crate) extern "C" fn result_from<S, E>(ptr: *mut c_void, res: sys::EDiscordResult, source: S)
where
    S: Into<E>,
{
    prevent_unwind!();

    unsafe { Box::from_raw(ptr as *mut Sender<Result<E>>) }
        .try_send(res.to_result().map(|()| source.into()))
        .expect(SEND_FAIL)
}

pub(crate) extern "C" fn result_from_ptr<S, E>(
    ptr: *mut c_void,
    res: sys::EDiscordResult,
    source_ptr: *mut S,
) where
    S: Into<E> + Copy,
{
    prevent_unwind!();

    unsafe { Box::from_raw(ptr as *mut Sender<Result<E>>) }
        .try_send(res.to_result().map(|()| unsafe { *source_ptr }.into()))
        .expect(SEND_FAIL)
}

pub(crate) extern "C" fn filter_relationship<F>(
    callback_ptr: *mut c_void,
    relationship_ptr: *mut sys::DiscordRelationship,
) -> bool
where
    F: FnMut(Relationship) -> bool,
{
    prevent_unwind!();

    let callback: *mut F = callback_ptr as *mut F;

    unsafe { (*callback)((*relationship_ptr).into()) }
}

pub(crate) extern "C" fn log(_: *mut c_void, level: sys::EDiscordLogLevel, message: *const i8) {
    use log::Level::*;

    prevent_unwind!();

    let level = match level {
        sys::DiscordLogLevel_Error => Error,
        sys::DiscordLogLevel_Warn => Warn,
        sys::DiscordLogLevel_Info => Info,
        sys::DiscordLogLevel_Debug => Debug,
        _ => Debug,
    };

    let message = unsafe { CStr::from_ptr(message) }.to_str().expect(NOT_UTF8);

    log::log!(level, "SDK: {}", message);
}
