pub(crate) fn with_tx<Tx, T>(ptr: *mut Tx, callback: impl FnOnce(&mut Tx) -> T) -> T {
    let tx = unsafe { &mut *ptr };
    callback(tx)
}

pub(crate) fn charbuf_to_str(charbuf: &[u8]) -> &str {
    let bytes = &charbuf[..charbuf_len(charbuf)];

    if cfg!(debug_assertions) {
        std::str::from_utf8(bytes).unwrap()
    } else {
        unsafe { std::str::from_utf8_unchecked(bytes) }
    }
}

pub(crate) fn charbuf_len(charbuf: &[u8]) -> usize {
    memchr::memchr(0, charbuf).unwrap_or_else(|| charbuf.len())
}

pub(crate) fn write_charbuf(charbuf: &mut [u8], value: &str) {
    let bytes = value.as_bytes();
    let len = bytes.len();

    debug_assert!(len <= charbuf.len());

    charbuf[..len].copy_from_slice(bytes);

    if len < charbuf.len() {
        charbuf[len] = 0;
    }
}

pub(crate) unsafe fn charptr_to_str<'a>(ptr: *const u8) -> &'a str {
    let bytes = std::ffi::CStr::from_ptr(ptr as *const i8).to_bytes();

    if cfg!(debug_assertions) {
        std::str::from_utf8(bytes).unwrap()
    } else {
        std::str::from_utf8_unchecked(bytes)
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
