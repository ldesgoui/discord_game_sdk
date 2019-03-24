use crate::Instance;
use discord_game_sdk_sys as sys;
use std::os::raw::{c_char, c_void};

/// Complete
pub unsafe extern "C" fn destroy(core: *mut sys::IDiscordCore) {
    let _inst: Box<Instance> = Box::from_raw(Instance::from_core(core));
}

pub unsafe extern "C" fn run_callbacks(core: *mut sys::IDiscordCore) -> sys::EDiscordResult {
    // TODO: store callbacks when calling async functions and run them here
    // TODO: add random delay?

    sys::DiscordResult_Ok
}

/// Complete
pub unsafe extern "C" fn set_log_hook(
    core: *mut sys::IDiscordCore,
    min_level: sys::EDiscordLogLevel,
    hook_data: *mut c_void,
    hook: Option<
        unsafe extern "C" fn(
            hook_data: *mut c_void,
            level: sys::EDiscordLogLevel,
            message: *const c_char,
        ),
    >,
) {
    let mut inst = Instance::from_core(core);

    inst.state.log_min_level = min_level;
    inst.state.log_hook = hook;
    inst.state.log_hook_data = hook_data;

    inst.log("Log example: Debug", sys::DiscordLogLevel_Debug);
    inst.log("Log example: Info", sys::DiscordLogLevel_Info);
    inst.log("Log example: Warn", sys::DiscordLogLevel_Warn);
    inst.log("Log example: Error", sys::DiscordLogLevel_Error);
}

/// Complete
pub unsafe extern "C" fn get_application_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordApplicationManager {
    &mut Instance::from_core(core).interfaces.application as *mut _
}

/// Complete
pub unsafe extern "C" fn get_user_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordUserManager {
    &mut Instance::from_core(core).interfaces.user as *mut _
}

/// Complete
pub unsafe extern "C" fn get_image_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordImageManager {
    &mut Instance::from_core(core).interfaces.image as *mut _
}

/// Complete
pub unsafe extern "C" fn get_activity_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordActivityManager {
    &mut Instance::from_core(core).interfaces.activity as *mut _
}

/// Complete
pub unsafe extern "C" fn get_relationship_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordRelationshipManager {
    &mut Instance::from_core(core).interfaces.relationship as *mut _
}

/// Complete
pub unsafe extern "C" fn get_lobby_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordLobbyManager {
    &mut Instance::from_core(core).interfaces.lobby as *mut _
}

/// Complete
pub unsafe extern "C" fn get_network_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordNetworkManager {
    &mut Instance::from_core(core).interfaces.network as *mut _
}

/// Complete
pub unsafe extern "C" fn get_overlay_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordOverlayManager {
    &mut Instance::from_core(core).interfaces.overlay as *mut _
}

/// Complete
pub unsafe extern "C" fn get_storage_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordStorageManager {
    &mut Instance::from_core(core).interfaces.storage as *mut _
}

/// Complete
pub unsafe extern "C" fn get_store_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordStoreManager {
    &mut Instance::from_core(core).interfaces.store as *mut _
}

/// Complete
pub unsafe extern "C" fn get_voice_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordVoiceManager {
    &mut Instance::from_core(core).interfaces.voice as *mut _
}

/// Complete
pub unsafe extern "C" fn get_achievement_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordAchievementManager {
    &mut Instance::from_core(core).interfaces.achievement as *mut _
}
