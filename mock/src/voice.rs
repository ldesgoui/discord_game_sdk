use crate::Instance;
use discord_game_sdk_sys as sys;
use std::os::raw::c_void;

/// Complete
pub unsafe extern "C" fn get_input_mode(
    manager: *mut sys::IDiscordVoiceManager,
    input_mode: *mut sys::DiscordInputMode,
) -> sys::EDiscordResult {
    let inst = Instance::from_voice(manager);

    *input_mode = inst.state.voice_input_mode;

    sys::DiscordResult_Ok
}

/// Complete
pub unsafe extern "C" fn set_input_mode(
    manager: *mut sys::IDiscordVoiceManager,
    input_mode: sys::DiscordInputMode,
    callback_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(callback_data: *mut c_void, result: sys::EDiscordResult)>,
) {
    let inst = Instance::from_voice(manager);

    inst.state.voice_input_mode = input_mode;

    callback.unwrap()(callback_data, sys::DiscordResult_Ok);
}

/// Complete
pub unsafe extern "C" fn is_self_mute(
    manager: *mut sys::IDiscordVoiceManager,
    mute: *mut bool,
) -> sys::EDiscordResult {
    let inst = Instance::from_voice(manager);

    *mute = inst.state.voice_self_mute;

    sys::DiscordResult_Ok
}

/// Complete
pub unsafe extern "C" fn set_self_mute(
    manager: *mut sys::IDiscordVoiceManager,
    mute: bool,
) -> sys::EDiscordResult {
    let inst = Instance::from_voice(manager);

    inst.state.voice_self_mute = mute;

    sys::DiscordResult_Ok
}

/// Complete
pub unsafe extern "C" fn is_self_deaf(
    manager: *mut sys::IDiscordVoiceManager,
    deaf: *mut bool,
) -> sys::EDiscordResult {
    let inst = Instance::from_voice(manager);

    *deaf = inst.state.voice_self_deaf;

    sys::DiscordResult_Ok
}

/// Complete
pub unsafe extern "C" fn set_self_deaf(
    manager: *mut sys::IDiscordVoiceManager,
    deaf: bool,
) -> sys::EDiscordResult {
    let inst = Instance::from_voice(manager);

    inst.state.voice_self_deaf = deaf;

    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn is_local_mute(
    manager: *mut sys::IDiscordVoiceManager,
    user_id: sys::DiscordSnowflake,
    mute: *mut bool,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn set_local_mute(
    manager: *mut sys::IDiscordVoiceManager,
    user_id: sys::DiscordSnowflake,
    mute: bool,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn get_local_volume(
    manager: *mut sys::IDiscordVoiceManager,
    user_id: sys::DiscordSnowflake,
    volume: *mut u8,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}

pub unsafe extern "C" fn set_local_volume(
    manager: *mut sys::IDiscordVoiceManager,
    user_id: sys::DiscordSnowflake,
    volume: u8,
) -> sys::EDiscordResult {
    sys::DiscordResult_Ok
}
