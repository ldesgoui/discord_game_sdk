use discord_game_sdk::*;

#[test]
fn compiles() {
    let mut gsdk = Discord::new(0).unwrap();
    let mut reader = gsdk.activity_events_reader();

    log::info!(
        "{:?}",
        gsdk.activity_events(&mut reader).collect::<Vec<_>>()
    );
}
