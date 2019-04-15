use crate::{callbacks::AnyCallback, event, sys};

/// Main interface with SDK
pub struct Discord<'a> {
    pub(crate) core: *mut sys::IDiscordCore,
    pub(crate) client_id: i64,
    #[allow(dead_code)]
    pub(crate) senders: Box<event::Senders>,
    pub(crate) receivers: event::Receivers,
    pub(crate) callbacks: Vec<Box<dyn AnyCallback + 'a>>,
}

impl<'a> Discord<'a> {
    pub(crate) fn register_callback(&mut self, callback: impl AnyCallback + 'a) {
        self.callbacks.push(Box::new(callback))
    }
}

impl<'a> std::fmt::Debug for Discord<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Discord")
            .field("ffi_ptr", &self.core)
            .field("client_id", &self.client_id)
            .finish()
    }
}
