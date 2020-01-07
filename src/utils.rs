use crate::discord::{Discord, DiscordInner};
use std::{ops::Deref, os::raw::c_uchar};

pub(crate) struct CallbackData<T> {
    pub(crate) discord: *const DiscordInner,
    pub(crate) callback: Box<dyn FnOnce(&Discord, T)>,
}

impl<T> CallbackData<T> {
    pub(crate) fn new(
        discord: &Discord,
        callback: impl 'static + FnOnce(&Discord, T),
    ) -> Box<Self> {
        Box::new(Self {
            discord: discord.0.deref() as *const _,
            callback: Box::new(callback),
        })
    }
}

pub(crate) struct MacroHelper<T> {
    pub(crate) core: *mut T,
}

impl<T> MacroHelper<T> {
    pub(crate) fn new(core: *mut T) -> (Self,) {
        (Self { core },)
    }
}

pub(crate) fn charbuf_to_str(charbuf: &[c_uchar]) -> &str {
    let bytes = &charbuf[..charbuf_len(charbuf)];

    if cfg!(debug_assertions) {
        std::str::from_utf8(bytes).unwrap()
    } else {
        unsafe { std::str::from_utf8_unchecked(bytes) }
    }
}

pub(crate) fn charbuf_len(charbuf: &[c_uchar]) -> usize {
    memchr::memchr(0, charbuf).unwrap_or_else(|| charbuf.len())
}

pub(crate) fn write_charbuf(charbuf: &mut [c_uchar], value: &str) {
    let bytes = value.as_bytes();
    let len = bytes.len();

    debug_assert!(len <= charbuf.len());

    charbuf[..len].copy_from_slice(bytes);

    if len < charbuf.len() {
        charbuf[len] = 0;
    }
}

pub(crate) fn charptr_to_str<'a>(ptr: *const c_uchar) -> &'a str {
    let bytes = unsafe { std::ffi::CStr::from_ptr(ptr as *const i8) }.to_bytes();

    if cfg!(debug_assertions) {
        std::str::from_utf8(bytes).unwrap()
    } else {
        unsafe { std::str::from_utf8_unchecked(bytes) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_charbuf() {
        run_test("");
        run_test("1");
        run_test("10 charact");
        run_test("64 characters 64 characters 64 characters 64 characters 64 chara");
    }

    #[test]
    #[should_panic]
    fn panic_test_write_charbuf() {
        run_test("65 characters 65 characters 65 characters 65 characters 65 charac");
    }

    fn run_test(val: &str) {
        let mut charbuf = [0u8; 64];

        write_charbuf(&mut charbuf, val);

        assert_eq!(charbuf_to_str(&charbuf), val);
    }
}
