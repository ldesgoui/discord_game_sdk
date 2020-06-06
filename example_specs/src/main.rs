#![allow(unused)]

use discord_game_sdk::{Activity, Discord, EventHandler};
use specs::{prelude::*, shrev::EventChannel};

// TODO: messy setup

fn main() {
    let client_id = std::env::var("DISCORD_CLIENT_ID").unwrap().parse().unwrap();
    let mut discord = Discord::new(client_id).unwrap();
    *discord.event_handler_mut() = Some(MyEventHandler);

    let mut world = World::new();

    let mut channel = EventChannel::<MyActivityUpdated>::default();
    let reader_id = channel.register_reader();
    world.insert(channel);

    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(DiscordSystem { discord, reader_id })
        .build();

    dispatcher.setup(&mut world);

    // the game loop
    for _ in 0..1000 {
        dispatcher.dispatch(&world);
        world.maintain();
    }
}

#[derive(Debug)]
struct DiscordSystem<'d> {
    discord: Discord<'d, MyEventHandler>,
    reader_id: ReaderId<MyActivityUpdated>,
}

impl DiscordSystem<'_> {
    fn update_activity(&mut self, channel: Read<'_, EventChannel<MyActivityUpdated>>) {
        match channel.read(&mut self.reader_id).last() {
            None => {}

            Some(MyActivityUpdated::Clear) => {
                self.discord.clear_activity(|_, result| {
                    if let Err(e) = result {
                        eprintln!("failed to clear activity: {}", e);
                    }
                });
            }

            Some(MyActivityUpdated::InMainMenu) => {
                self.discord.update_activity(
                    &Activity::empty().with_state("In Main Menu\0"),
                    |_, result| {
                        if let Err(e) = result {
                            eprintln!("failed to update activity: {}", e);
                        }
                    },
                );
            }

            Some(MyActivityUpdated::Playing { hero, map }) => {
                self.discord.update_activity(
                    &Activity::empty()
                        .with_state(&format!("Playing on {}\0", map))
                        .with_details(&format!("As {}\0", hero)),
                    |_, result| {
                        if let Err(e) = result {
                            eprintln!("failed to update activity: {}", e);
                        }
                    },
                );
            }
        }
    }
}

#[derive(Debug)]
struct MyEventHandler;

impl EventHandler for MyEventHandler {}

impl<'a, 'd> System<'a> for DiscordSystem<'d> {
    type SystemData = (Read<'a, EventChannel<MyActivityUpdated>>,);

    fn run(&mut self, (activity_updated,): Self::SystemData) {
        self.discord.run_callbacks().unwrap();

        self.update_activity(activity_updated);
    }
}

#[derive(Debug)]
enum MyActivityUpdated {
    Clear,
    InMainMenu,
    Playing { hero: String, map: String },
}
