macro_rules! ffi {
    // ffi!(self.destroy())
    ($self:ident . $method:ident ( $($args:expr),* $(,)? )) => {
        {
            assert!($self.core.$method.is_some());

            log::trace!(target: "discord_game_sdk", "calling FFI: {}", stringify!($method));

            $self.core.$method.unwrap()($self.core as *mut _, $( $args ),*)
        }
    };

    // ffi!(self.get_application_manager().get_current_locale())
    ($self:ident . $get_manager:ident () . $method:ident ( $($args:expr),* $(,)? )) => {
        {
            let manager = ffi!($self.$get_manager()).as_mut().unwrap();

            assert!(manager.$method.is_some());

            log::trace!(target: "discord_game_sdk", "calling FFI manager method: {}", stringify!($method));

            manager.$method.unwrap()(manager as *mut _, $( $args ),*)
        }
    };
}

macro_rules! prevent_unwind {
    () => {
        let hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(|info| {
            eprintln!();
            eprintln!("discord_game_sdk:");
            eprintln!("    The program has encountered a `panic` across FFI bounds,");
            eprintln!("    unwinding at this point would be undefined behavior,");
            eprintln!("    we will abort the process instead.");
            eprintln!("    Here are informations about the panic:");
            eprintln!();
            eprintln!("{}", info);
            eprintln!();

            std::process::abort();
        }));

        scopeguard::defer_on_success!({
            std::panic::set_hook(hook);
        });
    };
}
