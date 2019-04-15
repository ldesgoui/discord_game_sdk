use crate::panic_messages::{NOT_UTF8, NO_NUL};
use std::ffi::CStr;

pub(crate) fn slice_i8_to_u8(slice: &[i8]) -> &[u8] {
    unsafe { &*(slice as *const [i8] as *const [u8]) }
}

pub(crate) fn slice_u8_to_i8(slice: &[u8]) -> &[i8] {
    unsafe { &*(slice as *const [u8] as *const [i8]) }
}

pub(crate) fn cstr_to_str(slice: &[i8]) -> &str {
    let bytes = slice_i8_to_u8(slice);
    let end = memchr::memchr(0, bytes).expect(NO_NUL);

    unsafe { CStr::from_bytes_with_nul_unchecked(&bytes[..end + 1]) }
        .to_str()
        .expect(NOT_UTF8)
}
