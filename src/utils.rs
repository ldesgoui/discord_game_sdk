use crate::error::*;
use discord_game_sdk_sys as sys;
use std::os::raw::c_void;

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
