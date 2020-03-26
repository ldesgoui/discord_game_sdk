use discord_game_sdk::*;
use std::env;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    if let Err(error) = run() {
        log::error!("{}", error);
    }
}

fn run() -> anyhow::Result<()> {
    pretty_env_logger::try_init_timed_custom_env("IT_LOG")?;

    let client_id = env::var("DISCORD_CLIENT_ID")?.parse()?;
    log::info!("starting with client id {}", client_id);

    let mut alice = instance::<()>(client_id, 0)?;
    let mut bob = instance::<()>(client_id, 1)?;

    test_users(&mut alice, &mut bob)?;
    test_achievements(&mut alice, &mut bob)?;

    log::info!("successfully completed test suite");
    Ok(())
}

fn instance<'d, E: EventHandler>(client_id: i64, n: u32) -> anyhow::Result<Discord<'d, E>> {
    env::set_var("DISCORD_INSTANCE_ID", n.to_string());
    Ok(Discord::new(client_id)?)
}

fn test_users(alice: &mut Discord<'_, ()>, bob: &mut Discord<'_, ()>) -> anyhow::Result<()> {
    // TODO: Discord::user(id, callback)
    loop {
        alice.run_callbacks()?;
        bob.run_callbacks()?;

        let alice = alice.current_user();
        let bob = bob.current_user();

        if alice == Err(Error::NotFound) || bob == Err(Error::NotFound) {
            sleep(Duration::from_millis(100));
            continue;
        }

        let alice = alice?;
        let bob = bob?;

        log::info!("alice is {}#{}", alice.username(), alice.discriminator());
        log::info!("bob   is {}#{}", bob.username(), bob.discriminator());

        break;
    }

    log::info!("alice's premium: {:?}", alice.current_user_premium_kind()?);
    log::info!("alice's user flags: {:?}", alice.current_user_flags()?);

    Ok(())
}

fn test_achievements(alice: &mut Discord<()>, bob: &mut Discord<()>) -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel();

    alice.fetch_user_achievements(move |_alice, res| {
        log::info!("fetched user achievements");
        tx.send(res).unwrap();

        if res.is_err() {
            return;
        }
    });

    log::info!("waiting on user achievements");
    loop {
        alice.run_callbacks()?;
        bob.run_callbacks()?;

        match rx.try_recv() {
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => unreachable!(),
            Ok(res) => return Ok(res?),
        }

        sleep(Duration::from_millis(10));
    }
}
