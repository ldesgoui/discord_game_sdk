use std::ffi::CStr;

pub(crate) fn slice_i8_to_u8(slice: &[i8]) -> &[u8] {
    unsafe { &*(slice as *const [i8] as *const [u8]) }
}

pub(crate) fn slice_u8_to_i8(slice: &[u8]) -> &[i8] {
    unsafe { &*(slice as *const [u8] as *const [i8]) }
}


#[easy_ext::ext(CStrExt)]
impl CStr {
    pub fn from_bytes(bytes: &'_ [u8]) -> Result<&'_ CStr, std::ffi::FromBytesWithNulError> {
        if let Some(end) = memchr::memchr(0, bytes) {
            CStr::from_bytes_with_nul(&bytes[..end + 1])
        } else {
            CStr::from_bytes_with_nul(bytes)
        }
    }
}
