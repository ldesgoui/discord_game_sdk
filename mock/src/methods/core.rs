use crate::prelude::*;

#[no_mangle]
pub unsafe extern "C" fn DiscordCreate(
    version: sys::DiscordVersion,
    params: *mut sys::DiscordCreateParams,
    result: *mut *mut sys::IDiscordCore,
) -> sys::EDiscordResult {
    prevent_unwind!();

    logged_assert!(!result.is_null());

    let params = params.as_ref().unwrap();

    log::trace!("called DiscordCreate:");
    log::trace!("  - Client ID: {}", params.client_id);
    log::trace!(
        "  - Requires Discord to be running: {}",
        params.flags == sys::DiscordCreateFlags_Default as u64
    );
    log::trace!("  - SDK Version: {}", version);

    let inst = Instance::new(version, params);

    *result = Box::into_raw(Box::new(inst)) as *mut _;

    log::trace!("returning pointer to {:p}", *result);

    sys::DiscordResult_Ok
}

/// Complete
pub unsafe extern "C" fn destroy(core: *mut sys::IDiscordCore) {
    prevent_unwind!();

    let _inst: Box<Instance> = Box::from_raw(Instance::from_core(core));
}

pub unsafe extern "C" fn run_callbacks(core: *mut sys::IDiscordCore) -> sys::EDiscordResult {
    prevent_unwind!();

    let mut inst = Instance::from_core(core);
    inst.state.tick += 1;

    let mut i = 0;
    while i < inst.state.callbacks.len() {
        if inst.state.callbacks[i].0 <= inst.state.tick {
            let (_, mut cb) = inst.state.callbacks.remove(i);
            cb(inst)
        } else {
            i += 1
        }
    }

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
            message: *const i8,
        ),
    >,
) {
    prevent_unwind!();
    let mut inst = Instance::from_core(core);

    inst.state.log_min_level = min_level;
    inst.state.log_hook = hook;
    inst.state.log_hook_data = hook_data;

    inst.log(sys::DiscordLogLevel_Debug, "Log example: Debug");
    inst.log(sys::DiscordLogLevel_Info, "Log example: Info");
    inst.log(sys::DiscordLogLevel_Warn, "Log example: Warn");
    inst.log(sys::DiscordLogLevel_Error, "Log example: Error");
}

/// Complete
pub unsafe extern "C" fn get_application_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordApplicationManager {
    prevent_unwind!();
    &mut Instance::from_core(core).interfaces.applications as *mut _
}

/// Complete
pub unsafe extern "C" fn get_user_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordUserManager {
    prevent_unwind!();
    &mut Instance::from_core(core).interfaces.users as *mut _
}

/// Complete
pub unsafe extern "C" fn get_image_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordImageManager {
    prevent_unwind!();
    &mut Instance::from_core(core).interfaces.images as *mut _
}

/// Complete
pub unsafe extern "C" fn get_activity_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordActivityManager {
    prevent_unwind!();
    &mut Instance::from_core(core).interfaces.activities as *mut _
}

/// Complete
pub unsafe extern "C" fn get_relationship_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordRelationshipManager {
    prevent_unwind!();
    &mut Instance::from_core(core).interfaces.relationships as *mut _
}

/// Complete
pub unsafe extern "C" fn get_lobby_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordLobbyManager {
    prevent_unwind!();
    &mut Instance::from_core(core).interfaces.lobbies as *mut _
}

/// Complete
pub unsafe extern "C" fn get_network_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordNetworkManager {
    prevent_unwind!();
    &mut Instance::from_core(core).interfaces.networking as *mut _
}

/// Complete
pub unsafe extern "C" fn get_overlay_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordOverlayManager {
    prevent_unwind!();
    &mut Instance::from_core(core).interfaces.overlay as *mut _
}

/// Complete
pub unsafe extern "C" fn get_storage_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordStorageManager {
    prevent_unwind!();
    &mut Instance::from_core(core).interfaces.storage as *mut _
}

/// Complete
pub unsafe extern "C" fn get_store_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordStoreManager {
    prevent_unwind!();
    &mut Instance::from_core(core).interfaces.store as *mut _
}

/// Complete
pub unsafe extern "C" fn get_voice_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordVoiceManager {
    prevent_unwind!();
    &mut Instance::from_core(core).interfaces.voice as *mut _
}

/// Complete
pub unsafe extern "C" fn get_achievement_manager(
    core: *mut sys::IDiscordCore,
) -> *mut sys::IDiscordAchievementManager {
    prevent_unwind!();
    &mut Instance::from_core(core).interfaces.achievements as *mut _
}
