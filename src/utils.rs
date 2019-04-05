use crate::prelude::*;

pub(crate) trait FromSys: Sized {
    type Source;

    fn from_sys(source: &Self::Source) -> Self;

    unsafe fn from_sys_ptr(source: *const Self::Source) -> Self {
        Self::from_sys(source.as_ref().unwrap())
    }
}

pub(crate) unsafe fn string_from_cstr(ptr: *const c_char) -> String {
    CStr::from_ptr(ptr).to_str().unwrap().to_string()
}