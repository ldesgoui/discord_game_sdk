use crate::prelude::*;

/// # Activities
impl<'a> Discord<'a> {
    pub fn register_launch_command<S>(&mut self, command: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let cstring = std::ffi::CString::new(command.as_ref()).unwrap();

        unsafe {
            ffi!(self
                .get_activity_manager()
                .register_command(cstring.as_ptr()))
        }
        .to_result()
    }

    /// # Rate limit
    /// 5 updates per 20 seconds
    pub fn update_activity<F>(&mut self, activity_change: &ActivityChange, callback: F)
    where
        F: FnMut(Result<()>),
    {
        let mut activity = activity_change.to_sys();

        unsafe {
            ffi!(self.get_activity_manager().update_activity(
                &mut activity as *mut _,
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result::<F>)
            ))
        };
    }

    pub fn clear_activity<F>(&mut self, callback: F)
    where
        F: FnMut(Result<()>),
    {
        unsafe {
            ffi!(self.get_activity_manager().clear_activity(
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result::<F>)
            ))
        };
    }

    pub fn send_request_reply<F>(&mut self, user_id: i64, reply: RequestReply, callback: F)
    where
        F: FnMut(Result<()>),
    {
        unsafe {
            ffi!(self.get_activity_manager().send_request_reply(
                user_id,
                reply.to_sys(),
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result::<F>)
            ))
        };
    }

    pub fn send_invite<S, F>(&mut self, user_id: i64, action: Action, content: S, callback: F)
    where
        S: AsRef<str>,
        F: FnMut(Result<()>),
    {
        let content = std::ffi::CString::new(content.as_ref()).unwrap();

        unsafe {
            ffi!(self.get_activity_manager().send_invite(
                user_id,
                action.to_sys(),
                content.as_ptr(),
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result::<F>)
            ))
        };
    }

    pub fn accept_invite<F>(&mut self, user_id: i64, callback: F)
    where
        F: FnMut(Result<()>),
    {
        unsafe {
            ffi!(self.get_activity_manager().accept_invite(
                user_id,
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result::<F>)
            ))
        }
    }

    pub fn activity_events_reader(&mut self) -> shrev::ReaderId<event::Activity> {
        self.activity_channel.register_reader()
    }

    pub fn activity_events(
        &self,
        reader: &mut shrev::ReaderId<event::Activity>,
    ) -> shrev::EventIterator<event::Activity> {
        self.activity_channel.read(reader)
    }
}
