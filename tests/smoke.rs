use discord_game_sdk::*;

#[test]
fn compiles() {
    let _ = pretty_env_logger::try_init_custom_env("TEST_LOG");

    let mut gsdk = Discord::new(0).unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        if let Err(e) = gsdk.run_callbacks() {
            log::info!("run_callbacks error: {}", e);
            return;
        }
        gsdk.empty_event_receivers();
    }
}
