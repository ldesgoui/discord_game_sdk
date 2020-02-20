// Don't trust me

macro_rules! ffi {
    // ffi!(self.destroy())
    ($self:ident.$method:ident($($arg:expr),* $(,)?)) => {{
        let core = $self.inner().core;
        debug_assert!(!core.is_null());

        log::trace!("FFI: {}", stringify!($method));
        let function = (*core).$method;
        debug_assert!(function.is_some());
        let function = function.unwrap();

        function(core, $($arg),*)
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
        use crate::{discord::DiscordInner, utils::CallbackData};

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

            // SAFETY: We created the pointer just below
            let callback_data = Box::from_raw(callback_data as *mut CallbackData<$res>);

            // SAFETY:
            // We're duplicating the `Box<DiscordInner>`, this is safe:
            // - We're not mutating it, we're not dropping it
            // - No other part of the code will mutate it as `&mut Discord` is in the callstack
            let discord = Discord(Box::from_raw(callback_data.discord));

            (callback_data.callback)(&discord, $expr);

            // SAFETY: Not dropping our duplicated `Box<DiscordInner>`
            std::mem::forget(discord);
        }

        let callback = Box::new($callback);

        let callback_data = CallbackData {
            // SAFETY: We're passing the Box as raw pointer to duplicate it above
            // It won't be mutated or dropped
            discord: &*$self.0 as *const DiscordInner as *mut DiscordInner,
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

// TRACK:
// https://github.com/rust-lang/rust/issues/52652
// https://github.com/rust-lang/rust/issues/58760
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
