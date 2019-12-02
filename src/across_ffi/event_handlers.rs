use crate::{channels, panic_messages::NOT_UTF8};
use crossbeam_channel::Sender;
use std::ffi::{c_void, CStr};

unsafe fn send<Event>(senders: *mut c_void, ev: impl Into<Event>)
where
    channels::Senders: AsRef<Sender<Event>>,
{
    debug_assert!(!senders.is_null());

    let res = (&*(senders as *mut channels::Senders))
        .as_ref()
        .try_send(ev.into());

    debug_assert!(res.is_ok());
}

pub(crate) unsafe extern "C" fn empty<Event>(senders: *mut c_void)
where
    channels::Senders: AsRef<Sender<Event>>,
    Event: From<()>,
{
    prevent_unwind!();
    send(senders, ());
}

pub(crate) unsafe extern "C" fn plain<Event, Sys1>(senders: *mut c_void, sys1: Sys1)
where
    channels::Senders: AsRef<Sender<Event>>,
    Event: From<Sys1>,
    Sys1: Copy,
{
    prevent_unwind!();
    send(senders, sys1);
}

pub(crate) unsafe extern "C" fn ptr<Event, Sys1>(senders: *mut c_void, sys1: *mut Sys1)
where
    channels::Senders: AsRef<Sender<Event>>,
    Event: From<Sys1>,
    Sys1: Copy,
{
    prevent_unwind!();
    debug_assert!(!sys1.is_null());
    send(senders, *sys1);
}

pub(crate) unsafe extern "C" fn string<Event>(senders: *mut c_void, string: *const i8)
where
    channels::Senders: AsRef<Sender<Event>>,
    Event: From<String>,
{
    prevent_unwind!();
    debug_assert!(!string.is_null());

    let string = CStr::from_ptr(string).to_str().expect(NOT_UTF8).to_string();

    send(senders, string);
}

pub(crate) unsafe extern "C" fn plain_plain<Event, Sys1, Sys2>(
    senders: *mut c_void,
    sys1: Sys1,
    sys2: Sys2,
) where
    channels::Senders: AsRef<Sender<Event>>,
    Event: From<(Sys1, Sys2)>,
    Sys1: Copy,
    Sys2: Copy,
{
    prevent_unwind!();
    send(senders, (sys1, sys2));
}

pub(crate) unsafe extern "C" fn plain_plain_buffer<Event, Sys1, Sys2>(
    senders: *mut c_void,
    sys1: Sys1,
    sys2: Sys2,
    buf: *mut u8,
    buf_len: u32,
) where
    channels::Senders: AsRef<Sender<Event>>,
    Event: From<(Sys1, Sys2, Vec<u8>)>,
    Sys1: Copy,
    Sys2: Copy,
{
    prevent_unwind!();

    let buf = std::slice::from_raw_parts(buf, buf_len as usize).to_vec();

    send(senders, (sys1, sys2, buf));
}

pub(crate) unsafe extern "C" fn plain_plain_plain<Event, Sys1, Sys2, Sys3>(
    senders: *mut c_void,
    sys1: Sys1,
    sys2: Sys2,
    sys3: Sys3,
) where
    channels::Senders: AsRef<Sender<Event>>,
    Event: From<(Sys1, Sys2, Sys3)>,
    Sys1: Copy,
    Sys2: Copy,
    Sys3: Copy,
{
    prevent_unwind!();
    send(senders, (sys1, sys2, sys3));
}

pub(crate) unsafe extern "C" fn plain_ptr_ptr<Event, Sys1, Sys2, Sys3>(
    senders: *mut c_void,
    sys1: Sys1,
    sys2: *mut Sys2,
    sys3: *mut Sys3,
) where
    channels::Senders: AsRef<Sender<Event>>,
    Event: From<(Sys1, Sys2, Sys3)>,
    Sys1: Copy,
    Sys2: Copy,
    Sys3: Copy,
{
    prevent_unwind!();

    debug_assert!(!sys2.is_null());
    debug_assert!(!sys3.is_null());

    send(senders, (sys1, *sys2, *sys3));
}

pub(crate) unsafe extern "C" fn plain_plain_plain_buffer<Event, Sys1, Sys2, Sys3>(
    senders: *mut c_void,
    sys1: Sys1,
    sys2: Sys2,
    sys3: Sys3,
    buf: *mut u8,
    buf_len: u32,
) where
    channels::Senders: AsRef<Sender<Event>>,
    Event: From<(Sys1, Sys2, Sys3, Vec<u8>)>,
    Sys1: Copy,
    Sys2: Copy,
    Sys3: Copy,
{
    prevent_unwind!();

    let buf = std::slice::from_raw_parts(buf, buf_len as usize).to_vec();

    send(senders, (sys1, sys2, sys3, buf));
}
