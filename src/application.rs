use crate::error::*;
use crate::Discord;
use discord_game_sdk_sys as sys;
use std::os::raw::{c_char, c_void};

/// Application
impl Discord {
    pub fn get_current_locale(&self) -> Result<String> {
        let &mut mut locale: &mut sys::DiscordLocale = &mut [0; 128];

        ffi!(self
            .get_application_manager()
            .get_current_locale(&mut locale as *mut _))?;

        Ok(
            unsafe { std::ffi::CStr::from_ptr(&locale as *const _ as *const _) }
                .to_str()?
                .to_string(),
        )
    }

    pub fn get_current_branch(&self) -> Result<String> {
        let &mut mut branch: &mut sys::DiscordBranch = &mut [0; 4096];

        ffi!(self
            .get_application_manager()
            .get_current_branch(&mut branch as *mut _))?;

        Ok(
            unsafe { std::ffi::CStr::from_ptr(&branch as *const _ as *const _) }
                .to_str()?
                .to_string(),
        )
    }

    pub fn validate_or_exit<F>(&self, callback: &mut F)
    where
        F: FnMut(Result<()>),
    {
        let _ = ffi!(self
            .get_application_manager()
            .validate_or_exit(std::mem::transmute(callback), Some(validate_or_exit::<F>)));
    }

    //
    //    pub fn get_oauth2_token<F>(&self, callback: &mut F)
    //    where
    //        F: FnMut(Result<DiscordOAuth2Token>),
    //    {
    //        ffi!(self
    //            .get_application_manager()
    //            .get_oauth2_token(std::mem::transmute(callback), Some(get_oauth2_token::<F>)))
    //    }
}

extern "C" fn validate_or_exit<F>(data: *mut c_void, res: sys::EDiscordResult)
where
    F: FnMut(Result<()>) + Sized,
{
    // debug_assert!(!data.is_null());
    // let callback: *mut F = std::mem::transmute(data);

    // (*callback)(Error::from(res));
}

//extern "C" fn get_oauth2_token<F>(
//    data: *mut c_void,
//    res: sys::EDiscordResult,
//    token: *mut sys::DiscordOAuth2Token,
//) where
//    F: FnMut(Result<DiscordOAuth2Token>) + Sized,
//{
//    // debug_assert!(!data.is_null());
//    // let callback: *mut F = std::mem::transmute(data);
//
//    //match Error::from(res) {
//    //    Some(err) => (*callback)(Err(err)),
//    //    None => {
//    //        let token = token.as_ref().unwrap();
//    //        (*callback)(DiscordOAuth2Token::from(token))
//    //    }
//    //}
//}
//
//type DiscordOAuth2Token = ();
