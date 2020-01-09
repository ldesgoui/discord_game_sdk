macro_rules! ffi {
    // ffi!(self.destroy())
    ($self:ident.$method:ident($($arg:expr),* $(,)?)) => {{
        const MISSING_METHOD: &str =
            "[discord_game_sdk] received a NULL pointer where a valid pointer to a method is expected";

        log::trace!("FFI: {}", stringify!($method));
        let function = (*$self.0.core).$method.expect(MISSING_METHOD);

        function($self.0.core, $($arg),*)
    }};

    // ffi!(self.get_application_manager().get_current_locale())
    ($self:ident
        .$get_manager:ident ()
        .$method:ident($($arg:expr),* $(,)?)
    ) => {{
        const MISSING_METHOD: &str =
            "[discord_game_sdk] received a NULL pointer where a valid pointer to a method is expected";

        let manager = ffi!($self.$get_manager());

        log::trace!("FFI: .{}", stringify!($method));
        let function = (*manager).$method.expect(MISSING_METHOD);

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

        const MISSING_METHOD: &str =
            "[discord_game_sdk] received a NULL pointer where a valid pointer to a method is expected";

        let manager = ffi!($self.$get_manager());

        log::trace!("FFI: .{}", stringify!($method));
        let function = (*manager).$method.expect(MISSING_METHOD);

        unsafe extern "C" fn c_fn(
            callback_data: *mut std::ffi::c_void,
            $($param: $ty),*
        ) {
            prevent_unwind!();

            let callback_data = Box::from_raw(callback_data as *mut CallbackData<$res>);
            let discord = Discord(Box::from_raw(callback_data.discord as *mut _));

            (callback_data.callback)(&discord, $expr);

            std::mem::forget(discord);
        }

        let callback_data =
            Box::into_raw(Box::new(CallbackData::new($self, $callback))) as *mut _;

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
            use crate::discord::{Discord, DiscordInner};

            prevent_unwind!();

            let event_handler = &mut (*(inner as *mut DiscordInner)).event_handler;
            let discord = Discord(Box::from_raw(inner as *mut DiscordInner));

            event_handler.$method(
                &discord,
                $($expr),*
            );

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

// https://github.com/rust-lang/project-ffi-unwind
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
