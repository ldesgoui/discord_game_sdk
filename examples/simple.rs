use discord_game_sdk::*;

fn main() {
    pretty_env_logger::init();

    let client_id = 0;
    let mut gsdk = Discord::new(client_id).unwrap();

    gsdk.update_activity(
        &Activity::empty()
            .with_details("Trying stuff out")
            .with_state("using discord_game_sdk"),
        |_, _| {},
    );

    // Game loop
    loop {
        process_events(&gsdk);

        gsdk.empty_event_buffers();
        if let Err(e) = gsdk.run_callbacks() {
            log::info!("run_callbacks error: {}", e);
            return;
        }

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

#[rustfmt::skip]
fn process_events(gsdk: &Discord) {
    gsdk.recv_achievements_update().for_each(|x| { dbg!(x); });
    gsdk.recv_activities_join().for_each(|x| { dbg!(x); });
    gsdk.recv_activities_spectate().for_each(|x| { dbg!(x); });
    gsdk.recv_activities_request().for_each(|x| { dbg!(x); });
    gsdk.recv_activities_invite().for_each(|x| { dbg!(x); });
    gsdk.recv_lobbies_update().for_each(|x| { dbg!(x); });
    gsdk.recv_lobbies_delete().for_each(|x| { dbg!(x); });
    gsdk.recv_lobbies_member_connect().for_each(|x| { dbg!(x); });
    gsdk.recv_lobbies_member_update().for_each(|x| { dbg!(x); });
    gsdk.recv_lobbies_member_disconnect().for_each(|x| { dbg!(x); });
    gsdk.recv_lobbies_message().for_each(|x| { dbg!(x); });
    gsdk.recv_lobbies_speaking().for_each(|x| { dbg!(x); });
    gsdk.recv_lobbies_network_message().for_each(|x| { dbg!(x); });
    gsdk.recv_networking_message().for_each(|x| { dbg!(x); });
    gsdk.recv_networking_route_update().for_each(|x| { dbg!(x); });
    gsdk.recv_overlay_toggle().for_each(|x| { dbg!(x); });
    gsdk.recv_relationships_refresh().for_each(|x| { dbg!(x); });
    gsdk.recv_relationships_update().for_each(|x| { dbg!(x); });
    gsdk.recv_store_entitlement_create().for_each(|x| { dbg!(x); });
    gsdk.recv_store_entitlement_delete().for_each(|x| { dbg!(x); });
    gsdk.recv_current_user_update().for_each(|x| { dbg!(x); });
    gsdk.recv_voice_settings_update().for_each(|x| { dbg!(x); });
}
