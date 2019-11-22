pub(crate) fn from_charbuf(charbuf: &[i8]) -> &[u8] {
    unsafe { &*(charbuf as *const [i8] as *const [u8]) }
}

pub(crate) fn to_charbuf(bytes: &[u8]) -> &[i8] {
    unsafe { &*(bytes as *const [u8] as *const [i8]) }
}

pub(crate) fn charbuf_to_str(charbuf: &[i8]) -> &str {
    unsafe { std::str::from_utf8_unchecked(from_charbuf(charbuf)) }
}

pub(crate) fn charbuf_len(charbuf: &[i8]) -> usize {
    memchr::memchr(0, from_charbuf(charbuf)).unwrap_or_else(|| charbuf.len())
}

pub(crate) fn write_charbuf(charbuf: &mut [i8], value: &str) {
    let bytes = to_charbuf(value.as_bytes());
    let len = bytes.len();

    debug_assert!(len <= charbuf.len());

    charbuf[..len].copy_from_slice(bytes);

    if len < charbuf.len() {
        charbuf[len] = 0;
    }
}
