use discord_game_sdk::*;

fn main() {
    pretty_env_logger::init();

    let client_id = 0;
    let mut gsdk = Discord::new(client_id).unwrap();

    // Game loop
    loop {
        gsdk.empty_event_buffers();
        if let Err(e) = gsdk.run_callbacks() {
            log::info!("run_callbacks error: {}", e);
            return;
        }

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
