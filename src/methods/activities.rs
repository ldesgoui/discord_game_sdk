use crate::action::Action;
use crate::activity_change::ActivityChange;
use crate::event::ActivityEvent;
use crate::prelude::*;
use crate::request_reply::RequestReply;

/// Activities
impl Discord {
    pub fn register_launch_command<S>(&self, command: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let cstring = std::ffi::CString::new(command.as_ref()).map_err(DeveloperViolation::from)?;

        ffi!(self
            .get_activity_manager()
            .register_command(cstring.as_ptr()))
    }

    /// # Rate limit
    /// 5 updates per 20 seconds
    pub fn update_activity<F>(&self, activity_change: &ActivityChange, mut callback: F)
    where
        F: FnMut(Result<()>),
    {
        let mut activity = activity_change.to_sys().unwrap();

        let _ = ffi!(self.get_activity_manager().update_activity(
            &mut activity as *mut _,
            &callback as *const _ as *mut _,
            Some(simple_callback::<F>)
        ))
        .map_err(|e| callback(Err(e)));
    }

    pub fn clear_activity<F>(&self, mut callback: F)
    where
        F: FnMut(Result<()>),
    {
        let _ = ffi!(self
            .get_activity_manager()
            .clear_activity(&callback as *const _ as *mut _, Some(simple_callback::<F>)))
        .map_err(|e| callback(Err(e)));
    }

    pub fn send_request_reply<F>(&self, user_id: i64, reply: RequestReply, mut callback: F)
    where
        F: FnMut(Result<()>),
    {
        let _ = ffi!(self.get_activity_manager().send_request_reply(
            user_id,
            reply.to_sys(),
            &callback as *const _ as *mut _,
            Some(simple_callback::<F>)
        ))
        .map_err(|e| callback(Err(e)));
    }

    pub fn send_invite<S, F>(&self, user_id: i64, action: Action, content: S, mut callback: F)
    where
        S: AsRef<str>,
        F: FnMut(Result<()>),
    {
        let _ = std::ffi::CString::new(content.as_ref())
            .map_err(DeveloperViolation::from)
            .map_err(Error::from)
            .and_then(|cstring| {
                ffi!(self.get_activity_manager().send_invite(
                    user_id,
                    action.to_sys(),
                    cstring.as_ptr(),
                    &callback as *const _ as *mut _,
                    Some(simple_callback::<F>)
                ))
            })
            .map_err(|e| callback(Err(e)));
    }

    pub fn accept_invite<F>(&self, user_id: i64, mut callback: F)
    where
        F: FnMut(Result<()>),
    {
        let _ = ffi!(self.get_activity_manager().accept_invite(
            user_id,
            &callback as *const _ as *mut _,
            Some(simple_callback::<F>)
        ))
        .map_err(|e| callback(Err(e)));
    }

    pub fn activity_events_reader(&mut self) -> shrev::ReaderId<ActivityEvent> {
        self.activity_events.register_reader()
    }

    pub fn activity_events(
        &self,
        reader: &mut shrev::ReaderId<ActivityEvent>,
    ) -> shrev::EventIterator<ActivityEvent> {
        self.activity_events.read(reader)
    }
}
