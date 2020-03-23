use crate::{
    discord::{Discord, DiscordInner},
    events, CreateFlags, EventHandler, UserAchievement,
};
use std::{cell::UnsafeCell, marker::PhantomData};

mod ffi;

impl<E> Discord<'_, E> {
    pub(crate) fn mock() -> Self
    where
        E: EventHandler,
    {
        let mut instance = Discord(Box::into_raw(Box::new(DiscordInner {
            _invariant_lifetime: PhantomData,

            core: std::ptr::null_mut(),
            client_id: 0,
            event_handler: UnsafeCell::new(None),

            achievement_events: events::achievement::<E>(),
            activity_events: events::activity::<E>(),
            lobby_events: events::lobby::<E>(),
            network_events: events::network::<E>(),
            overlay_events: events::overlay::<E>(),
            relationship_events: events::relationship::<E>(),
            store_events: events::store::<E>(),
            user_events: events::user::<E>(),
            voice_events: events::voice::<E>(),
        })));

        let params = instance.create_params(CreateFlags::Default.into());

        instance.inner_mut().core = unsafe { ffi::create_mock(params) };

        instance
    }
}

#[test]
fn miri_tests() {
    struct E;

    impl EventHandler for E {
        fn on_user_achievement_update(
            &mut self,
            discord: &Discord<'_, Self>,
            user_achievement: &UserAchievement,
        ) {
            for a in discord.iter_user_achievements() {
                let a = a.unwrap();
                eprintln!(
                    "in event_handler {}: {}%",
                    a.achievement_id(),
                    a.percent_complete()
                );
            }

            if user_achievement.percent_complete() == 99 {
                discord.set_user_achievement(
                    user_achievement.achievement_id(),
                    100,
                    |discord, _res| {
                        for a in discord.iter_user_achievements() {
                            let a = a.unwrap();
                            eprintln!(
                                "in event_handler in set {}: {}%",
                                a.achievement_id(),
                                a.percent_complete()
                            );
                        }
                    },
                );
            }
        }
    }

    let mut discord = Discord::mock();
    *discord.event_handler_mut() = Some(E);

    discord.fetch_user_achievements(|discord, _res| {
        discord.set_user_achievement(0, 99, |discord, _res| {
            for a in discord.iter_user_achievements() {
                let a = a.unwrap();
                eprintln!("in fetch in set {}: {}%", a.achievement_id(), a.percent_complete());
            }
        });
    });

    for _ in 0..100 {
        discord.run_callbacks().unwrap();
    }

    discord.fetch_user_achievements(|discord, _res| {
        discord.set_user_achievement(0, 99, |_discord, _res| {});
    });
}
