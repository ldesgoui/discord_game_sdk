use crate::sys;
use std::ffi::c_void;

const CORE: &sys::IDiscordCore = &sys::IDiscordCore {
    destroy: {
        unsafe extern "C" fn destroy(_: *mut sys::IDiscordCore) {
            while let Some(closure) = STATE.as_mut().unwrap().queue.pop() {
                closure()
            }

            drop(STATE.take());
        }

        Some(destroy)
    },

    run_callbacks: {
        unsafe extern "C" fn run_callbacks(_: *mut sys::IDiscordCore) -> sys::EDiscordResult {
            while let Some(closure) = STATE.as_mut().unwrap().queue.pop() {
                closure()
            }

            sys::DiscordResult_Ok
        }

        Some(run_callbacks)
    },

    get_achievement_manager: {
        unsafe extern "C" fn get_achievement_manager(
            _: *mut sys::IDiscordCore,
        ) -> *mut sys::IDiscordAchievementManager {
            ACHIEVEMENT_MANAGER as *const _ as *mut _
        }

        Some(get_achievement_manager)
    },

    set_log_hook: None,
    get_application_manager: None,
    get_user_manager: None,
    get_image_manager: None,
    get_activity_manager: None,
    get_relationship_manager: None,
    get_lobby_manager: None,
    get_network_manager: None,
    get_overlay_manager: None,
    get_storage_manager: None,
    get_store_manager: None,
    get_voice_manager: None,
};

const ACHIEVEMENT_MANAGER: &sys::IDiscordAchievementManager = &sys::IDiscordAchievementManager {
    set_user_achievement: {
        unsafe extern "C" fn set_user_achievements(
            _: *mut sys::IDiscordAchievementManager,
            achievement_id: sys::DiscordSnowflake,
            percent_complete: u8,
            callback_data: *mut c_void,
            callback: Option<unsafe extern "C" fn(*mut c_void, sys::EDiscordResult)>,
        ) {
            STATE.as_mut().unwrap().queue.push(Box::new(move || {
                let state = STATE.as_mut().unwrap();

                for achievement in state.achievements.iter_mut() {
                    if achievement.achievement_id == achievement_id {
                        achievement.percent_complete = percent_complete;

                        (*state.params.achievement_events)
                            .on_user_achievement_update
                            .unwrap()(
                            state.params.event_data, achievement as *const _ as *mut _
                        )
                    }
                }

                callback.unwrap()(callback_data, sys::DiscordResult_Ok);
            }))
        }

        Some(set_user_achievements)
    },

    fetch_user_achievements: {
        unsafe extern "C" fn fetch_user_achievements(
            _: *mut sys::IDiscordAchievementManager,
            callback_data: *mut c_void,
            callback: Option<unsafe extern "C" fn(*mut c_void, sys::EDiscordResult)>,
        ) {
            STATE.as_mut().unwrap().queue.push(Box::new(move || {
                callback.unwrap()(callback_data, sys::DiscordResult_Ok);
            }))
        }

        Some(fetch_user_achievements)
    },

    get_user_achievement: {
        unsafe extern "C" fn get_user_achievement(
            _: *mut sys::IDiscordAchievementManager,
            user_achievement_id: sys::DiscordSnowflake,
            user_achievement: *mut sys::DiscordUserAchievement,
        ) -> sys::EDiscordResult {
            for achievement in &STATE.as_ref().unwrap().achievements {
                if achievement.achievement_id == user_achievement_id {
                    *user_achievement = *achievement;

                    return sys::DiscordResult_Ok;
                }
            }

            sys::DiscordResult_NotFound
        }

        Some(get_user_achievement)
    },

    count_user_achievements: {
        unsafe extern "C" fn count_user_achievements(
            _: *mut sys::IDiscordAchievementManager,
            count: *mut i32,
        ) {
            *count = STATE.as_ref().unwrap().achievements.len() as i32;
        }
        Some(count_user_achievements)
    },

    get_user_achievement_at: {
        unsafe extern "C" fn get_user_achievement_at(
            _: *mut sys::IDiscordAchievementManager,
            index: i32,
            user_achievement: *mut sys::DiscordUserAchievement,
        ) -> sys::EDiscordResult {
            *user_achievement = STATE.as_ref().unwrap().achievements[index as usize];

            sys::DiscordResult_Ok
        }

        Some(get_user_achievement_at)
    },
};

#[derive(Default)]
struct State {
    params: sys::DiscordCreateParams,
    achievements: Vec<sys::DiscordUserAchievement>,
    queue: Vec<Box<dyn FnOnce()>>,
}

static mut STATE: Option<State> = None;

pub(crate) unsafe fn create_mock(params: sys::DiscordCreateParams) -> *mut sys::IDiscordCore {
    if STATE.is_some() {
        panic!("can only hold one instance lol");
    }

    STATE = Some(State {
        params,
        achievements: (0..10)
            .map(|achievement_id| sys::DiscordUserAchievement {
                user_id: 0,
                achievement_id,
                percent_complete: 0,
                unlocked_at: [0; 64],
            })
            .collect(),
        ..Default::default()
    });

    CORE as *const _ as *mut _
}
