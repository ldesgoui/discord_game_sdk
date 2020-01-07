use crate::{
    sys, utils::charptr_to_str, Activity, Entitlement, Relationship, User, UserAchievement,
};

pub(crate) const ACHIEVEMENT: &sys::IDiscordAchievementEvents = &sys::IDiscordAchievementEvents {
    on_user_achievement_update: event_handler!(
        |user_achievement: *mut sys::DiscordUserAchievement| {
            EventHandler::on_user_achievement_update(&*(user_achievement as *mut UserAchievement))
        }
    ),
};

pub(crate) const ACTIVITY: &sys::IDiscordActivityEvents = &sys::IDiscordActivityEvents {
    on_activity_join: event_handler!(|secret: *const u8| {
        EventHandler::on_activity_join(charptr_to_str(secret))
    }),

    on_activity_spectate: event_handler!(|secret: *const u8| {
        EventHandler::on_activity_spectate(charptr_to_str(secret))
    }),

    on_activity_join_request: event_handler!(|user: *mut sys::DiscordUser| {
        EventHandler::on_activity_join_request(&*(user as *mut User))
    }),

    on_activity_invite: event_handler!(
        |kind: sys::EDiscordActivityActionType,
         user: *mut sys::DiscordUser,
         activity: *mut sys::DiscordActivity| {
            EventHandler::on_activity_invite(
                kind.into(),
                &*(user as *mut User),
                &*(activity as *mut Activity),
            )
        }
    ),
};

pub(crate) const LOBBY: &sys::IDiscordLobbyEvents = &sys::IDiscordLobbyEvents {
    on_lobby_update: event_handler!(|lobby_id: sys::DiscordLobbyId| {
        EventHandler::on_lobby_update(lobby_id)
    }),

    on_lobby_delete: event_handler!(|lobby_id: sys::DiscordLobbyId, reason: u32| {
        EventHandler::on_lobby_delete(lobby_id, reason)
    }),

    on_member_connect: event_handler!(
        |lobby_id: sys::DiscordLobbyId, member_id: sys::DiscordUserId| {
            EventHandler::on_member_connect(lobby_id, member_id)
        }
    ),

    on_member_update: event_handler!(
        |lobby_id: sys::DiscordLobbyId, member_id: sys::DiscordUserId| {
            EventHandler::on_member_update(lobby_id, member_id)
        }
    ),

    on_member_disconnect: event_handler!(
        |lobby_id: sys::DiscordLobbyId, member_id: sys::DiscordUserId| {
            EventHandler::on_member_disconnect(lobby_id, member_id)
        }
    ),

    on_lobby_message: event_handler!(|lobby_id: sys::DiscordLobbyId,
                                      member_id: sys::DiscordUserId,
                                      data: *mut u8,
                                      data_len: u32| {
        EventHandler::on_lobby_message(
            lobby_id,
            member_id,
            std::slice::from_raw_parts(data, data_len as usize),
        )
    }),

    on_speaking: event_handler!(|lobby_id: sys::DiscordLobbyId,
                                 member_id: sys::DiscordUserId,
                                 speaking: bool| {
        EventHandler::on_speaking(lobby_id, member_id, speaking)
    }),

    on_network_message: event_handler!(|lobby_id: sys::DiscordLobbyId,
                                        member_id: sys::DiscordUserId,
                                        channel_id: sys::DiscordNetworkChannelId,
                                        data: *mut u8,
                                        data_len: u32| {
        EventHandler::on_lobby_network_message(
            lobby_id,
            member_id,
            channel_id,
            std::slice::from_raw_parts(data, data_len as usize),
        )
    }),
};

pub(crate) const NETWORK: &sys::IDiscordNetworkEvents = &sys::IDiscordNetworkEvents {
    on_message: event_handler!(|peer_id: sys::DiscordNetworkPeerId,
                                channel_id: sys::DiscordNetworkChannelId,
                                data: *mut u8,
                                data_len: u32| {
        EventHandler::on_network_message(
            peer_id,
            channel_id,
            std::slice::from_raw_parts(data, data_len as usize),
        )
    }),

    on_route_update: event_handler!(|route: *const u8| {
        EventHandler::on_network_route_update(charptr_to_str(route))
    }),
};

pub(crate) const OVERLAY: &sys::IDiscordOverlayEvents = &sys::IDiscordOverlayEvents {
    on_toggle: event_handler!(|locked: bool| EventHandler::on_overlay_toggle(!locked)),
};

pub(crate) const RELATIONSHIP: &sys::IDiscordRelationshipEvents =
    &sys::IDiscordRelationshipEvents {
        on_refresh: event_handler!(|| EventHandler::on_relationships_refresh()),

        on_relationship_update: event_handler!(|relationship: *mut sys::DiscordRelationship| {
            EventHandler::on_relationship_update(&*(relationship as *mut Relationship))
        }),
    };

pub(crate) const STORE: &sys::IDiscordStoreEvents = &sys::IDiscordStoreEvents {
    on_entitlement_create: event_handler!(|entitlement: *mut sys::DiscordEntitlement| {
        EventHandler::on_entitlement_create(&*(entitlement as *mut Entitlement))
    }),

    on_entitlement_delete: event_handler!(|entitlement: *mut sys::DiscordEntitlement| {
        EventHandler::on_entitlement_delete(&*(entitlement as *mut Entitlement))
    }),
};

pub(crate) const USER: &sys::IDiscordUserEvents = &sys::IDiscordUserEvents {
    on_current_user_update: event_handler!(|| EventHandler::on_current_user_update()),
};

pub(crate) const VOICE: &sys::IDiscordVoiceEvents = &sys::IDiscordVoiceEvents {
    on_settings_update: event_handler!(|| EventHandler::on_voice_settings_update()),
};
