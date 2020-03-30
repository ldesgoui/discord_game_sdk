use crate::{
    discord::{Discord, DiscordInner},
    sys, utils, Activity, Entitlement, EventHandler, Relationship, User, UserAchievement,
};
use std::{ffi::c_void, mem::ManuallyDrop};

fn with_event_handler<E>(
    inner: *mut c_void,
    callback: impl FnOnce(&mut E, &Discord<'_, E>) + std::panic::UnwindSafe,
) {
    utils::abort_on_panic(|| {
        debug_assert!(!inner.is_null());

        let discord = &ManuallyDrop::new(Discord(inner as *mut DiscordInner<'_, E>));

        // SAFETY: Mutating through an immutable reference
        // - `discord.0.event_handler` is an `UnsafeCell`, inner mutation is legal
        // - No other part of the code can safely mutate it as they require `&mut DiscordInner`
        // - `EventHandler` can mutate itself during method but not `&Discord`
        let mut event_handler = unsafe { (*discord.inner().event_handler.get()).take() };

        if let Some(event_handler) = event_handler.as_mut() {
            callback(event_handler, discord);
        }

        // SAFETY: See previous
        unsafe {
            (*discord.inner().event_handler.get()) = event_handler;
        }
    })
}

pub(crate) fn achievement<E: EventHandler>() -> sys::IDiscordAchievementEvents {
    sys::IDiscordAchievementEvents {
        on_user_achievement_update: {
            extern "C" fn on_user_achievement_update<E: EventHandler>(
                inner: *mut c_void,
                user_achievement: *mut sys::DiscordUserAchievement,
            ) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_user_achievement_update(discord, unsafe {
                        &*(user_achievement as *const UserAchievement)
                    })
                })
            }

            Some(on_user_achievement_update::<E>)
        },
    }
}

pub(crate) fn activity<E: EventHandler>() -> sys::IDiscordActivityEvents {
    sys::IDiscordActivityEvents {
        on_activity_join: {
            extern "C" fn on_activity_join<E: EventHandler>(inner: *mut c_void, secret: *const u8) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_activity_join(discord, unsafe { utils::charptr_to_str(secret) })
                })
            }

            Some(on_activity_join::<E>)
        },

        on_activity_spectate: {
            extern "C" fn on_activity_spectate<E: EventHandler>(
                inner: *mut c_void,
                secret: *const u8,
            ) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_activity_spectate(discord, unsafe { utils::charptr_to_str(secret) })
                })
            }

            Some(on_activity_spectate::<E>)
        },

        on_activity_join_request: {
            extern "C" fn on_activity_join_request<E: EventHandler>(
                inner: *mut c_void,
                user: *mut sys::DiscordUser,
            ) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_activity_join_request(discord, unsafe { &*(user as *const User) })
                })
            }

            Some(on_activity_join_request::<E>)
        },

        on_activity_invite: {
            extern "C" fn on_activity_invite<E: EventHandler>(
                inner: *mut c_void,
                kind: sys::EDiscordActivityActionType,
                user: *mut sys::DiscordUser,
                activity: *mut sys::DiscordActivity,
            ) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_activity_invite(
                        discord,
                        kind.into(),
                        unsafe { &*(user as *const User) },
                        unsafe { &*(activity as *const Activity) },
                    )
                })
            }

            Some(on_activity_invite::<E>)
        },
    }
}

pub(crate) fn lobby<E: EventHandler>() -> sys::IDiscordLobbyEvents {
    sys::IDiscordLobbyEvents {
        on_lobby_update: {
            extern "C" fn on_lobby_update<E: EventHandler>(
                inner: *mut c_void,
                lobby_id: sys::DiscordLobbyId,
            ) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_lobby_update(discord, lobby_id)
                })
            }

            Some(on_lobby_update::<E>)
        },

        on_lobby_delete: {
            extern "C" fn on_lobby_delete<E: EventHandler>(
                inner: *mut c_void,
                lobby_id: sys::DiscordLobbyId,
                reason: u32,
            ) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_lobby_delete(discord, lobby_id, reason)
                })
            }

            Some(on_lobby_delete::<E>)
        },

        on_member_connect: {
            extern "C" fn on_member_connect<E: EventHandler>(
                inner: *mut c_void,
                lobby_id: sys::DiscordLobbyId,
                member_id: sys::DiscordUserId,
            ) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_member_connect(discord, lobby_id, member_id)
                })
            }

            Some(on_member_connect::<E>)
        },

        on_member_update: {
            extern "C" fn on_member_update<E: EventHandler>(
                inner: *mut c_void,
                lobby_id: sys::DiscordLobbyId,
                member_id: sys::DiscordUserId,
            ) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_member_update(discord, lobby_id, member_id)
                })
            }

            Some(on_member_update::<E>)
        },

        on_member_disconnect: {
            extern "C" fn on_member_disconnect<E: EventHandler>(
                inner: *mut c_void,
                lobby_id: sys::DiscordLobbyId,
                member_id: sys::DiscordUserId,
            ) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_member_disconnect(discord, lobby_id, member_id)
                })
            }

            Some(on_member_disconnect::<E>)
        },

        on_lobby_message: {
            extern "C" fn on_lobby_message<E: EventHandler>(
                inner: *mut c_void,
                lobby_id: sys::DiscordLobbyId,
                member_id: sys::DiscordUserId,
                data: *mut u8,
                data_len: u32,
            ) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_lobby_message(discord, lobby_id, member_id, unsafe {
                        std::slice::from_raw_parts(data, data_len as usize)
                    })
                })
            }

            Some(on_lobby_message::<E>)
        },

        on_speaking: {
            extern "C" fn on_speaking<E: EventHandler>(
                inner: *mut c_void,
                lobby_id: sys::DiscordLobbyId,
                member_id: sys::DiscordUserId,
                speaking: bool,
            ) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_speaking(discord, lobby_id, member_id, speaking)
                })
            }

            Some(on_speaking::<E>)
        },

        on_network_message: {
            extern "C" fn on_network_message<E: EventHandler>(
                inner: *mut c_void,
                lobby_id: sys::DiscordLobbyId,
                member_id: sys::DiscordUserId,
                channel_id: sys::DiscordNetworkChannelId,
                data: *mut u8,
                data_len: u32,
            ) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_lobby_network_message(discord, lobby_id, member_id, channel_id, unsafe {
                        std::slice::from_raw_parts(data, data_len as usize)
                    })
                })
            }

            Some(on_network_message::<E>)
        },
    }
}

pub(crate) fn network<E: EventHandler>() -> sys::IDiscordNetworkEvents {
    sys::IDiscordNetworkEvents {
        on_message: {
            extern "C" fn on_message<E: EventHandler>(
                inner: *mut c_void,
                peer_id: sys::DiscordNetworkPeerId,
                channel_id: sys::DiscordNetworkChannelId,
                data: *mut u8,
                data_len: u32,
            ) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_network_message(discord, peer_id, channel_id, unsafe {
                        std::slice::from_raw_parts(data, data_len as usize)
                    })
                })
            }

            Some(on_message::<E>)
        },

        on_route_update: {
            extern "C" fn on_route_update<E: EventHandler>(inner: *mut c_void, route: *const u8) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_network_route_update(discord, unsafe { utils::charptr_to_str(route) })
                })
            }

            Some(on_route_update::<E>)
        },
    }
}

pub(crate) fn overlay<E: EventHandler>() -> sys::IDiscordOverlayEvents {
    sys::IDiscordOverlayEvents {
        on_toggle: {
            extern "C" fn on_toggle<E: EventHandler>(inner: *mut c_void, locked: bool) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_overlay_toggle(discord, !locked)
                })
            }

            Some(on_toggle::<E>)
        },
    }
}

pub(crate) fn relationship<E: EventHandler>() -> sys::IDiscordRelationshipEvents {
    sys::IDiscordRelationshipEvents {
        on_refresh: {
            extern "C" fn on_refresh<E: EventHandler>(inner: *mut c_void) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_relationships_refresh(discord)
                })
            }

            Some(on_refresh::<E>)
        },

        on_relationship_update: {
            extern "C" fn on_relationship_update<E: EventHandler>(
                inner: *mut c_void,
                relationship: *mut sys::DiscordRelationship,
            ) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_relationship_update(discord, unsafe {
                        &*(relationship as *const Relationship)
                    })
                })
            }

            Some(on_relationship_update::<E>)
        },
    }
}

pub(crate) fn store<E: EventHandler>() -> sys::IDiscordStoreEvents {
    sys::IDiscordStoreEvents {
        on_entitlement_create: {
            extern "C" fn on_entitlement_create<E: EventHandler>(
                inner: *mut c_void,
                entitlement: *mut sys::DiscordEntitlement,
            ) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_entitlement_create(discord, unsafe {
                        &*(entitlement as *const Entitlement)
                    })
                })
            }

            Some(on_entitlement_create::<E>)
        },

        on_entitlement_delete: {
            extern "C" fn on_entitlement_delete<E: EventHandler>(
                inner: *mut c_void,
                entitlement: *mut sys::DiscordEntitlement,
            ) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_entitlement_delete(discord, unsafe {
                        &*(entitlement as *const Entitlement)
                    })
                })
            }

            Some(on_entitlement_delete::<E>)
        },
    }
}

pub(crate) fn user<E: EventHandler>() -> sys::IDiscordUserEvents {
    sys::IDiscordUserEvents {
        on_current_user_update: {
            extern "C" fn on_current_user_update<E: EventHandler>(inner: *mut c_void) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_current_user_update(discord)
                })
            }

            Some(on_current_user_update::<E>)
        },
    }
}

pub(crate) fn voice<E: EventHandler>() -> sys::IDiscordVoiceEvents {
    sys::IDiscordVoiceEvents {
        on_settings_update: {
            extern "C" fn on_settings_update<E: EventHandler>(inner: *mut c_void) {
                with_event_handler(inner, |eh: &mut E, discord| {
                    eh.on_voice_settings_update(discord)
                })
            }

            Some(on_settings_update::<E>)
        },
    }
}
