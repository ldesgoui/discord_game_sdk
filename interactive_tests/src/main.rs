use discord_game_sdk::*;
use std::env;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;

type D<'r, 'd> = &'r mut Discord<'d, ()>;

fn main() {
    if let Err(error) = run() {
        log::error!("{}", error);
    }
}

fn run() -> anyhow::Result<()> {
    pretty_env_logger::try_init_custom_env("IT_LOG")?;

    let client_id = env::var("DISCORD_CLIENT_ID")?.parse()?;
    log::info!("starting with client id {}", client_id);

    let mut bob = instance::<()>(client_id, 0)?;
    let mut alice = instance::<()>(client_id, 0)?;

    log::info!("running callbacks for a second");
    for _ in 0..100 {
        alice.run_callbacks()?;
        bob.run_callbacks()?;

        sleep(Duration::from_millis(10));
    }

    test_users(&mut alice, &mut bob)?;

    test_achievements(&mut alice, &mut bob)?;
    test_activities(&mut alice, &mut bob)?;

    log::info!("successfully completed test suite");
    Ok(())
}

fn instance<'d, E: EventHandler>(client_id: i64, n: u32) -> anyhow::Result<Discord<'d, E>> {
    env::set_var("DISCORD_INSTANCE_ID", n.to_string());
    Ok(Discord::new(client_id)?)
}

fn test_users(alice: D, bob: D) -> anyhow::Result<()> {
    // TODO: Discord::user(id, callback)
    log::info!("testing user methods");

    loop {
        alice.run_callbacks()?;
        bob.run_callbacks()?;

        let alice = alice.current_user();
        let bob = bob.current_user();

        if alice == Err(Error::NotFound) || bob == Err(Error::NotFound) {
            log::trace!("user not found");
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

fn test_achievements(alice: D, bob: D) -> anyhow::Result<()> {
    // TODO:
    // set achievement
    // achievement event
    // single achievement
    log::info!("testing achievement methods");

    let (orig_tx, rx) = mpsc::channel();

    let tx = orig_tx.clone();
    alice.fetch_user_achievements(move |_alice, res| {
        log::info!("fetched user achievements");
        tx.send(res).unwrap();
    });

    log::info!("waiting for user achievements to get fetched");
    wait(&rx, alice, bob)?;

    log::info!(
        "iterating over {} user achievements",
        alice.user_achievement_count()
    );
    for ach in alice.iter_user_achievements() {
        log::info!("ach {:?}", ach);
    }

    Ok(())
}

fn test_activities(alice: D, bob: D) -> anyhow::Result<()> {
    log::info!("testing activity methods");

    alice.register_launch_command("https://example.com")?;

    let (tx, rx) = mpsc::channel();

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    alice.update_activity(
        &Activity::empty()
            .with_state("hey")
            .with_details("testing")
            .with_start_time(now)
            .with_end_time(now + 600),
        move |_alice, res| {
            log::info!("activity updated");
            tx.send(res).unwrap();
        },
    );

    wait(&rx, alice, bob)?;

    Ok(())
}

fn wait<T>(rx: &mpsc::Receiver<Result<T>>, alice: D, bob: D) -> Result<T> {
    loop {
        alice.run_callbacks()?;
        bob.run_callbacks()?;

        match rx.try_recv() {
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => unreachable!(),
            Ok(res) => return res,
        }

        sleep(Duration::from_millis(10));
    }
}
