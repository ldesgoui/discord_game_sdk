use discord_game_sdk::*;

fn main() {
    pretty_env_logger::init();

    let client_id = std::env::var("DISCORD_CLIENT_ID").unwrap().parse().unwrap();

    let mut gsdk = Discord::new(client_id).unwrap();

    *gsdk.event_handler_mut() = Some(LogEvents);

    log::info!("got {:#?}", gsdk);

    gsdk.update_activity(
        &Activity::empty()
            .with_details("Trying stuff out")
            .with_state("using discord_game_sdk"),
        |res| {
            log::info!("update_activity: {:?}", res);
            panic!()
        },
    );

    // Game loop
    loop {
        if let Err(e) = gsdk.run_callbacks() {
            log::info!("run_callbacks error: {}", e);
            return;
        }

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

#[derive(Debug)]
struct LogEvents;

impl EventHandler for LogEvents {
    fn on_user_achievement_update(
        &mut self,
        _: &Discord<Self>,
        user_achievement: &UserAchievement,
    ) {
        log::info!("on user achievement update: {:#?}", user_achievement)
    }

    fn on_activity_join(&mut self, _: &Discord<Self>, secret: &str) {
        log::info!("on activity join: {:#?}", secret)
    }
    fn on_activity_spectate(&mut self, _: &Discord<Self>, secret: &str) {
        log::info!("on activity spectate: {:#?}", secret)
    }
    fn on_activity_join_request(&mut self, _: &Discord<Self>, user: &User) {
        log::info!("on activity join request: {:#?}", user)
    }
    fn on_activity_invite(
        &mut self,
        _: &Discord<Self>,
        action: Action,
        user: &User,
        activity: &Activity,
    ) {
        log::info!(
            "on activity invite: {:#?} {:#?} {:#?}",
            action,
            user,
            activity
        )
    }

    /*
    fn on_lobby_update(&mut self, _: &Discord<Self>, lobby_id: LobbyID) {}
    fn on_lobby_delete(&mut self, _: &Discord<Self>, lobby_id: LobbyID, reason: u32) {}
    fn on_member_connect(&mut self, _: &Discord<Self>, lobby_id: LobbyID, member_id: UserID) {}
    fn on_member_update(&mut self, _: &Discord<Self>, lobby_id: LobbyID, member_id: UserID) {}
    fn on_member_disconnect(&mut self, _: &Discord<Self>, lobby_id: LobbyID, member_id: UserID) {}
    fn on_lobby_message(&mut self, _: &Discord<Self>, lobby_id: LobbyID, member_id: UserID, data: &[u8]) {
    }
    fn on_speaking(&mut self, _: &Discord<Self>, lobby_id: LobbyID, member_id: UserID, speaking: bool) {}
    fn on_lobby_network_message(
        &mut self,
        _: &Discord<Self>,
        lobby_id: LobbyID,
        member_id: UserID,
        channel_id: NetworkChannelID,
        data: &[u8],
    ) {
    }

    fn on_network_message(
        &mut self,
        _: &Discord<Self>,
        peer_id: NetworkPeerID,
        channel_id: NetworkChannelID,
        data: &[u8],
    ) {
    }
    fn on_network_route_update(&mut self, _: &Discord<Self>, route: &str) {}
    */

    fn on_overlay_toggle(&mut self, _: &Discord<Self>, closed: bool) {
        log::info!("on overlay toggle: closed == {}", closed);
    }

    fn on_relationships_refresh(&mut self, _: &Discord<Self>) {
        log::info!("on relationships refresh");
    }
    fn on_relationship_update(&mut self, _: &Discord<Self>, relationship: &Relationship) {
        log::info!("on relationship update: {:#?}", relationship);
    }

    fn on_entitlement_create(&mut self, _: &Discord<Self>, entitlement: &Entitlement) {
        log::info!("on entitlement create: {:?}", entitlement);
    }
    fn on_entitlement_delete(&mut self, _: &Discord<Self>, entitlement: &Entitlement) {
        log::info!("on entitlement delete: {:?}", entitlement);
    }

    fn on_current_user_update(&mut self, _: &Discord<Self>) {
        log::info!("on current user update");
    }

    fn on_voice_settings_update(&mut self, _: &Discord<Self>) {
        log::info!("on voice settings update");
    }
}
