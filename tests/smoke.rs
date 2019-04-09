use discord_game_sdk::*;

#[test]
fn compiles() {
    let _ = env_logger::try_init_from_env("TEST_LOG");

    let mut gsdk = Discord::new(0).unwrap();

    gsdk.validate_or_exit(|gsdk, res| {
        log::info!("{:?}", gsdk.current_branch());
    });

    std::thread::sleep(std::time::Duration::from_secs(1));
    gsdk.run_callbacks();
}
