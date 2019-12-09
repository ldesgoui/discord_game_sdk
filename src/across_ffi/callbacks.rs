use crate::{panic_messages::NOT_UTF8, sys, to_result::ToResult, Relationship};
use crossbeam_channel::Sender;
use std::ffi::{c_void, CStr};

unsafe fn send<T>(ptr: *mut c_void, msg: T) {
    let res = Box::from_raw(ptr as *mut Sender<T>).try_send(msg);

    debug_assert!(res.is_ok())
}

pub(crate) unsafe extern "C" fn result(ptr: *mut c_void, res: sys::EDiscordResult) {
    prevent_unwind!();

    send(ptr, res.to_result());
}

pub(crate) unsafe extern "C" fn result_string(
    ptr: *mut c_void,
    res: sys::EDiscordResult,
    cstr: *const i8,
) {
    prevent_unwind!();

    send(
        ptr,
        res.to_result()
            .map(|()| CStr::from_ptr(cstr).to_str().expect(NOT_UTF8).to_string()),
    );
}

pub(crate) unsafe extern "C" fn result_bytes(
    ptr: *mut c_void,
    res: sys::EDiscordResult,
    buffer_ptr: *mut u8,
    len: u32,
) {
    prevent_unwind!();

    debug_assert!(!buffer_ptr.is_null());

    send(
        ptr,
        res.to_result()
            .map(|()| std::slice::from_raw_parts(buffer_ptr, len as usize).to_vec()),
    );
}

pub(crate) unsafe extern "C" fn result_from<S, E>(
    ptr: *mut c_void,
    res: sys::EDiscordResult,
    source: S,
) where
    S: Into<E>,
{
    prevent_unwind!();

    send(ptr, res.to_result().map(|()| source.into()));
}

pub(crate) unsafe extern "C" fn result_from_ptr<S, E>(
    ptr: *mut c_void,
    res: sys::EDiscordResult,
    source_ptr: *mut S,
) where
    S: Into<E> + Copy,
{
    prevent_unwind!();

    send(ptr, res.to_result().map(|()| (*source_ptr).into()));
}

pub(crate) unsafe extern "C" fn filter_relationship<F>(
    callback_ptr: *mut c_void,
    relationship_ptr: *mut sys::DiscordRelationship,
) -> bool
where
    F: FnMut(Relationship) -> bool,
{
    prevent_unwind!();

    let callback: *mut F = callback_ptr as *mut F;

    (*callback)((*relationship_ptr).into())
}

pub(crate) unsafe extern "C" fn log(
    _: *mut c_void,
    level: sys::EDiscordLogLevel,
    message: *const i8,
) {
    use log::Level::*;

    prevent_unwind!();

    let level = match level {
        sys::DiscordLogLevel_Error => Error,
        sys::DiscordLogLevel_Warn => Warn,
        sys::DiscordLogLevel_Info => Info,
        sys::DiscordLogLevel_Debug => Debug,
        _ => Debug,
    };

    let message = CStr::from_ptr(message).to_str().expect(NOT_UTF8);

    log::log!(level, "SDK: {}", message);
}
