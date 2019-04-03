use crate::prelude::*;

pub(crate) trait FromSys: Sized {
    type Source;

    fn from_sys(source: &Self::Source) -> Result<Self>;

    fn from_sys_ptr(source: *const Self::Source) -> Result<Self> {
        Self::from_sys(unsafe { source.as_ref() }.ok_or(BindingsViolation::NullPointer)?)
    }
}

pub(crate) fn from_cstr<'a>(cstr: *const c_char) -> Result<&'a str> {
    unsafe { std::ffi::CStr::from_ptr(cstr) }
        .to_str()
        .map_err(BindingsViolation::from)
        .map_err(Error::from)
}

pub(crate) extern "C" fn simple_callback<F>(data: *mut c_void, res: sys::EDiscordResult)
where
    F: FnMut(Result<()>) + Sized,
{
    if data.is_null() {
        log::error!("SDK invoked callback with null");
        return;
    }
    let callback: &mut F = unsafe { &mut *(data as *mut _) };

    callback(res.to_result());
}
