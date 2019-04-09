use crate::prelude::*;

pub struct State {
    pub version: sys::DiscordVersion,
    pub params: sys::DiscordCreateParams,

    pub log_min_level: sys::EDiscordLogLevel,
    pub log_hook_data: *mut c_void,
    pub log_hook: Option<
        unsafe extern "C" fn(
            hook_data: *mut c_void,
            level: sys::EDiscordLogLevel,
            message: *const i8,
        ),
    >,

    pub overlay_opened: bool,

    pub voice_input_mode: sys::DiscordInputMode,
    pub voice_self_mute: bool,
    pub voice_self_deaf: bool,

    pub tick: u64,
    pub callbacks: Vec<(u64, Box<dyn FnMut(&mut Instance)>)>,
}

impl State {
    pub fn new(version: sys::DiscordVersion, params: &sys::DiscordCreateParams) -> Self {
        Self {
            version,
            params: sys::DiscordCreateParams { ..*params },
            log_min_level: 0,
            log_hook_data: std::ptr::null_mut(),
            log_hook: None,
            overlay_opened: false,
            voice_input_mode: sys::DiscordInputMode {
                type_: sys::DiscordInputModeType_VoiceActivity,
                shortcut: [0; 256],
            },
            voice_self_mute: false,
            voice_self_deaf: false,
            tick: 0,
            callbacks: Vec::new(),
        }
    }

    pub fn queue(&mut self, rel_tick: u64, callback: impl FnMut(&mut Instance) + 'static) {
        self.callbacks
            .push((self.tick + rel_tick, Box::new(callback)))
    }
}
