pub(crate) fn slice_i8_to_u8(slice: &[i8]) -> &[u8] {
    unsafe { &*(slice as *const [i8] as *const [u8]) }
}

pub(crate) fn slice_u8_to_i8(slice: &[u8]) -> &[i8] {
    unsafe { &*(slice as *const [u8] as *const [i8]) }
}
