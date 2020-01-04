use discord_game_sdk::*;

fn main() {
    pretty_env_logger::init();

    let client_id = std::env::var("DISCORD_APPLICATION_ID")
        .unwrap()
        .parse()
        .unwrap();
    let mut gsdk = Discord::new(client_id).unwrap();

    gsdk.update_activity(
        &Activity::empty()
            .with_details("Trying stuff out")
            .with_state("using discord_game_sdk"),
        |_, res| log::info!("update_activity: {:?}", res),
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
    macro_rules! ev {
        ($name: ident) => {
            gsdk.$name().for_each(|ev| log::info!("{}: {:#?}", stringify!($name), ev));
        }
    }

    ev!(recv_achievements_update);
    ev!(recv_activities_join);
    ev!(recv_activities_spectate);
    ev!(recv_activities_request);
    ev!(recv_activities_invite);
    ev!(recv_lobbies_update);
    ev!(recv_lobbies_delete);
    ev!(recv_lobbies_member_connect);
    ev!(recv_lobbies_member_update);
    ev!(recv_lobbies_member_disconnect);
    ev!(recv_lobbies_message);
    ev!(recv_lobbies_speaking);
    ev!(recv_lobbies_network_message);
    ev!(recv_networking_message);
    ev!(recv_networking_route_update);
    ev!(recv_overlay_toggle);
    ev!(recv_relationships_refresh);
    ev!(recv_relationships_update);
    ev!(recv_store_entitlement_create);
    ev!(recv_store_entitlement_delete);
    ev!(recv_current_user_update);
    ev!(recv_voice_settings_update);
}
