use crate::{
    discord::{Discord, DiscordInner},
    sys, utils, Activity, Entitlement, EventHandler, Relationship, User, UserAchievement,
};
use std::{ffi::c_void, ops::DerefMut};

fn with_event_handler(inner: *mut c_void, callback: impl FnOnce(&mut dyn EventHandler, &Discord)) {
    prevent_unwind!();

    debug_assert!(!inner.is_null());

    // SAFETY:
    // We're duplicating the `Box<DiscordInner>`, this is safe:
    // - We're not mutating it, we're not dropping it
    // - No other part of the code will mutate it as `&mut Discord` is in the callstack
    let discord = unsafe { Discord(Box::from_raw(inner as *mut DiscordInner)) };

    // SAFETY: Mutation through immutable reference
    // - `discord.0.event_handler` is an `UnsafeCell`, inner mutation is legal
    // - No other part of the code can safely mutate it as they require `&mut DiscordInner`
    // - `EventHandler` can mutate itself during method but not `&Discord`
    let mut event_handler = unsafe { (*discord.0.event_handler.get()).take() };

    if let Some(event_handler) = event_handler.as_mut() {
        callback(event_handler.deref_mut(), &discord);
    }

    // SAFETY: See previous
    unsafe {
        (*discord.0.event_handler.get()) = event_handler;
    }

    // SAFETY: Not dropping our duplicated `Box<DiscordInner>`
    std::mem::forget(discord);
}

pub(crate) const ACHIEVEMENT: &sys::IDiscordAchievementEvents = &sys::IDiscordAchievementEvents {
    on_user_achievement_update: {
        extern "C" fn on_user_achievement_update(
            inner: *mut c_void,
            user_achievement: *mut sys::DiscordUserAchievement,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_user_achievement_update(discord, unsafe {
                    &*(user_achievement as *const UserAchievement)
                })
            })
        }

        Some(on_user_achievement_update)
    },
};

pub(crate) const ACTIVITY: &sys::IDiscordActivityEvents = &sys::IDiscordActivityEvents {
    on_activity_join: {
        extern "C" fn on_activity_join(inner: *mut c_void, secret: *const u8) {
            with_event_handler(inner, |eh, discord| {
                eh.on_activity_join(discord, unsafe { utils::charptr_to_str(secret) })
            })
        }

        Some(on_activity_join)
    },

    on_activity_spectate: {
        extern "C" fn on_activity_spectate(inner: *mut c_void, secret: *const u8) {
            with_event_handler(inner, |eh, discord| {
                eh.on_activity_spectate(discord, unsafe { utils::charptr_to_str(secret) })
            })
        }

        Some(on_activity_spectate)
    },

    on_activity_join_request: {
        extern "C" fn on_activity_join_request(inner: *mut c_void, user: *mut sys::DiscordUser) {
            with_event_handler(inner, |eh, discord| {
                eh.on_activity_join_request(discord, unsafe { &*(user as *const User) })
            })
        }

        Some(on_activity_join_request)
    },

    on_activity_invite: {
        extern "C" fn on_activity_invite(
            inner: *mut c_void,
            kind: sys::EDiscordActivityActionType,
            user: *mut sys::DiscordUser,
            activity: *mut sys::DiscordActivity,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_activity_invite(
                    discord,
                    kind.into(),
                    unsafe { &*(user as *const User) },
                    unsafe { &*(activity as *const Activity) },
                )
            })
        }

        Some(on_activity_invite)
    },
};

pub(crate) const LOBBY: &sys::IDiscordLobbyEvents = &sys::IDiscordLobbyEvents {
    on_lobby_update: {
        extern "C" fn on_lobby_update(inner: *mut c_void, lobby_id: sys::DiscordLobbyId) {
            with_event_handler(inner, |eh, discord| eh.on_lobby_update(discord, lobby_id))
        }

        Some(on_lobby_update)
    },

    on_lobby_delete: {
        extern "C" fn on_lobby_delete(
            inner: *mut c_void,
            lobby_id: sys::DiscordLobbyId,
            reason: u32,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_lobby_delete(discord, lobby_id, reason)
            })
        }

        Some(on_lobby_delete)
    },

    on_member_connect: {
        extern "C" fn on_member_connect(
            inner: *mut c_void,
            lobby_id: sys::DiscordLobbyId,
            member_id: sys::DiscordUserId,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_member_connect(discord, lobby_id, member_id)
            })
        }

        Some(on_member_connect)
    },

    on_member_update: {
        extern "C" fn on_member_update(
            inner: *mut c_void,
            lobby_id: sys::DiscordLobbyId,
            member_id: sys::DiscordUserId,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_member_update(discord, lobby_id, member_id)
            })
        }

        Some(on_member_update)
    },

    on_member_disconnect: {
        extern "C" fn on_member_disconnect(
            inner: *mut c_void,
            lobby_id: sys::DiscordLobbyId,
            member_id: sys::DiscordUserId,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_member_disconnect(discord, lobby_id, member_id)
            })
        }

        Some(on_member_disconnect)
    },

    on_lobby_message: {
        extern "C" fn on_lobby_message(
            inner: *mut c_void,
            lobby_id: sys::DiscordLobbyId,
            member_id: sys::DiscordUserId,
            data: *mut u8,
            data_len: u32,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_lobby_message(discord, lobby_id, member_id, unsafe {
                    std::slice::from_raw_parts(data, data_len as usize)
                })
            })
        }

        Some(on_lobby_message)
    },

    on_speaking: {
        extern "C" fn on_speaking(
            inner: *mut c_void,
            lobby_id: sys::DiscordLobbyId,
            member_id: sys::DiscordUserId,
            speaking: bool,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_speaking(discord, lobby_id, member_id, speaking)
            })
        }

        Some(on_speaking)
    },

    on_network_message: {
        extern "C" fn on_network_message(
            inner: *mut c_void,
            lobby_id: sys::DiscordLobbyId,
            member_id: sys::DiscordUserId,
            channel_id: sys::DiscordNetworkChannelId,
            data: *mut u8,
            data_len: u32,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_lobby_network_message(discord, lobby_id, member_id, channel_id, unsafe {
                    std::slice::from_raw_parts(data, data_len as usize)
                })
            })
        }

        Some(on_network_message)
    },
};

pub(crate) const NETWORK: &sys::IDiscordNetworkEvents = &sys::IDiscordNetworkEvents {
    on_message: {
        extern "C" fn on_message(
            inner: *mut c_void,
            peer_id: sys::DiscordNetworkPeerId,
            channel_id: sys::DiscordNetworkChannelId,
            data: *mut u8,
            data_len: u32,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_network_message(discord, peer_id, channel_id, unsafe {
                    std::slice::from_raw_parts(data, data_len as usize)
                })
            })
        }

        Some(on_message)
    },

    on_route_update: {
        extern "C" fn on_route_update(inner: *mut c_void, route: *const u8) {
            with_event_handler(inner, |eh, discord| {
                eh.on_network_route_update(discord, unsafe { utils::charptr_to_str(route) })
            })
        }

        Some(on_route_update)
    },
};

pub(crate) const OVERLAY: &sys::IDiscordOverlayEvents = &sys::IDiscordOverlayEvents {
    on_toggle: {
        extern "C" fn on_toggle(inner: *mut c_void, locked: bool) {
            with_event_handler(inner, |eh, discord| eh.on_overlay_toggle(discord, !locked))
        }

        Some(on_toggle)
    },
};

pub(crate) const RELATIONSHIP: &sys::IDiscordRelationshipEvents =
    &sys::IDiscordRelationshipEvents {
        on_refresh: {
            extern "C" fn on_refresh(inner: *mut c_void) {
                with_event_handler(inner, |eh, discord| eh.on_relationships_refresh(discord))
            }

            Some(on_refresh)
        },

        on_relationship_update: {
            extern "C" fn on_relationship_update(
                inner: *mut c_void,
                relationship: *mut sys::DiscordRelationship,
            ) {
                with_event_handler(inner, |eh, discord| {
                    eh.on_relationship_update(discord, unsafe {
                        &*(relationship as *const Relationship)
                    })
                })
            }

            Some(on_relationship_update)
        },
    };

pub(crate) const STORE: &sys::IDiscordStoreEvents = &sys::IDiscordStoreEvents {
    on_entitlement_create: {
        extern "C" fn on_entitlement_create(
            inner: *mut c_void,
            entitlement: *mut sys::DiscordEntitlement,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_entitlement_create(discord, unsafe { &*(entitlement as *const Entitlement) })
            })
        }

        Some(on_entitlement_create)
    },

    on_entitlement_delete: {
        extern "C" fn on_entitlement_delete(
            inner: *mut c_void,
            entitlement: *mut sys::DiscordEntitlement,
        ) {
            with_event_handler(inner, |eh, discord| {
                eh.on_entitlement_delete(discord, unsafe { &*(entitlement as *const Entitlement) })
            })
        }

        Some(on_entitlement_delete)
    },
};

pub(crate) const USER: &sys::IDiscordUserEvents = &sys::IDiscordUserEvents {
    on_current_user_update: {
        extern "C" fn on_current_user_update(inner: *mut c_void) {
            with_event_handler(inner, |eh, discord| eh.on_current_user_update(discord))
        }

        Some(on_current_user_update)
    },
};

pub(crate) const VOICE: &sys::IDiscordVoiceEvents = &sys::IDiscordVoiceEvents {
    on_settings_update: {
        extern "C" fn on_settings_update(inner: *mut c_void) {
            with_event_handler(inner, |eh, discord| eh.on_voice_settings_update(discord))
        }

        Some(on_settings_update)
    },
};
