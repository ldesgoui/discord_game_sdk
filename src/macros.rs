macro_rules! ffi {
    // ffi!(self.destroy())
    ($self:ident.$method:ident($($arg:expr),* $(,)?)) => {{
        debug_assert!(!$self.0.core.is_null());

        log::trace!("FFI: {}", stringify!($method));
        let function = (*$self.0.core).$method;

        debug_assert!(function.is_some());

        let function = function.unwrap();

        function($self.0.core, $($arg),*)
    }};

    // ffi!(self.get_application_manager().get_current_locale())
    ($self:ident
        .$get_manager:ident ()
        .$method:ident($($arg:expr),* $(,)?)
    ) => {{
        let manager = ffi!($self.$get_manager());

        debug_assert!(!manager.is_null());

        log::trace!("FFI: .{}", stringify!($method));
        let function = (*manager).$method;

        debug_assert!(function.is_some());

        let function = function.unwrap();

        function(manager, $($arg),*)
    }};


    ($self:ident
        .$get_manager:ident()
        .$method:ident($($arg:expr),* $(,)?)
        .and_then(|$($param:ident: $ty:ty),* $(,)?| {
            $callback:ident::<$res:ty>($expr:expr $(,)?)
        })
    ) => {{
        use crate::utils::CallbackData;
        use std::ops::Deref;

        let manager = ffi!($self.$get_manager());

        debug_assert!(!manager.is_null());

        log::trace!("FFI: .{}", stringify!($method));
        let function = (*manager).$method;

        debug_assert!(function.is_some());

        let function = function.unwrap();

        unsafe extern "C" fn c_fn(
            callback_data: *mut std::ffi::c_void,
            $($param: $ty),*
        ) {
            prevent_unwind!();

            debug_assert!(!callback_data.is_null());

            // SAFETY: Legal, we created the pointer just below
            let callback_data = Box::from_raw(callback_data as *mut CallbackData<$res>);

            // SAFETY: Legal (Mutable aliasing)
            // - We don't mutate the Box<DiscordInner> (no dropping either)
            // - We only pass a &Discord
            // - At this point in time, we are *always* in Discord::run_callbacks
            // - Which means we're in posession of &mut Discord
            // - And downgrading to a &Discord and passing it to another function is fine
            let discord = Discord(Box::from_raw(callback_data.discord as *mut _));

            (callback_data.callback)(&discord, $expr);

            // SAFETY: We cannot drop Box<DiscordInner>
            std::mem::forget(discord);
        }

        let callback = Box::new($callback);

        let callback_data = CallbackData {
            discord: $self.0.deref() as *const _,
            callback,
        };

        // SAFETY: We create the pointer here
        let callback_data = Box::into_raw(Box::new(callback_data)) as *mut _;

        function(manager, $($arg, )* callback_data, Some(c_fn))
    }};

    ($self:ident
        .$get_manager:ident()
        .$method:ident($($arg:expr),* $(,)?)
        .and_then(|$($param:ident: $ty:ty),* $(,)?| $callback:ident::<$res:ty>($expr:expr $(,)?))
    ) => {
        ffi!($self
             .$get_manager()
             .$method($($arg),*)
             .and_then(|$($param: $ty),*| { $callback::<$res>($expr) }))
    };
}

macro_rules! event_handler {
    // on_activity_invite: event_handler!(
    //     |kind: sys::EDiscordActivityActionType,
    //      user: *mut sys::DiscordUser,
    //      activity: *mut sys::DiscordActivity| {
    //         EventHandler::on_activity_invite(
    //             kind.into(),
    //             &*(user as *mut User),
    //             &*(activity as *mut Activity),
    //         )
    //     }
    // ),
    (|$($param:ident: $ty:ty),* $(,)?| {
        EventHandler::$method:ident(
            $( $expr:expr ),* $(,)?
        )
    }) => {{
        unsafe extern "C" fn $method(
            inner: *mut std::ffi::c_void,
            $($param: $ty),*
        ) {
            use crate::{discord::{Discord, DiscordInner}, utils::VoidEvents};

            prevent_unwind!();

            debug_assert!(!inner.is_null());

            // SAFETY: Legal (Mutable aliasing)
            // - We don't drop the Box<DiscordInner>
            // - At this point in time, we are *always* in Discord::run_callbacks
            // - Which means we're in posession of &mut Discord
            // - We are absolutely sure that only one of the &mut are used at one time
            let mut discord = Discord(Box::from_raw(inner as *mut DiscordInner));

            // We're sure that the event handler won't be used through discord,
            // we replace it with a blank value for now
            // PERF: Heap-allocs a VTable
            let mut event_handler = discord.set_event_handler(Box::new(VoidEvents));

            event_handler.$method(
                &discord,
                $($expr),*
            );

            // We're sure this will go through, otherwise we're panicking
            // and therefore about to abort
            discord.set_event_handler(event_handler);

            // SAFETY: We cannot drop Box<DiscordInner>
            std::mem::forget(discord);
        }

        Some($method)
    }};

    (|$($param:ident: $ty:ty),* $(,)?| EventHandler::$method:ident($($expr:expr),* $(,)?)) => {
        event_handler!(|$($param: $ty),*| {
            EventHandler::$method($($expr),*)
        })
    };

    (|| EventHandler::$method:ident()) => {
        event_handler!(| | { EventHandler::$method() })
    };
}

// TODO: https://github.com/rust-lang/project-ffi-unwind
macro_rules! prevent_unwind {
    () => {
        const ACROSS_FFI: &str = "[discord_game_sdk]
            The program has encountered a `panic` across FFI bounds, unwinding at this
            point would be undefined behavior, we will abort the process instead.
            Please report this issue to https://github.com/ldesgoui/discord_game_sdk
            Here is the panic message:";

        let hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(|info| {
            log::error!("panic across FFI bounds: {}", info);
            eprintln!("\n{}\n\n{}\n", ACROSS_FFI, info);
            std::process::abort();
        }));

        scopeguard::defer_on_success!({
            std::panic::set_hook(hook);
        });
    };
}
