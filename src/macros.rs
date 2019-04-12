macro_rules! log { ($level:expr, $($arg:tt)+) => (log::log!(target: "discord_game_sdk", $level, $($arg)+)) }
macro_rules! error { ($($arg:tt)+) => (log!(log::Level::Error, $($arg)+)) }
macro_rules! warn { ($($arg:tt)+) => (log!(log::Level::Warn, $($arg)+)) }
macro_rules! info { ($($arg:tt)+) => (log!(log::Level::Info, $($arg)+)) }
macro_rules! debug { ($($arg:tt)+) => (log!(log::Level::Debug, $($arg)+)) }
macro_rules! trace { ($($arg:tt)+) => (log!(log::Level::Trace, $($arg)+)) }

macro_rules! ffi {
    // ffi!(self.destroy())
    ($self:ident . $method:ident ($($args:expr),* $(,)?)) => {{
        trace!("FFI: {}", stringify!($method));
        let function = (*$self.core).$method.unwrap();
        function($self.core, $( $args ),*)
    }};

    // ffi!(self.get_application_manager().get_current_locale())
    ($self:ident .  $get_manager:ident () .  $method:ident ($($args:expr),* $(,)?)) => {{
        let manager = ffi!($self.$get_manager());
        trace!("FFI:     {}", stringify!($method));
        let function = (*manager).$method.unwrap();
        function(manager, $( $args ),*)
    }};

    // ffi!(self.get_activity_manager().accept_invite(user_id)(callback))
    ($self:ident .  $get_manager:ident () .  $method:ident ($($args:expr),* $(,)?) ($callback:expr $(,)?)) => {{
        let manager = ffi!($self.$get_manager());
        trace!("FFI:     {}", stringify!($method));
        let function = (*manager).$method.unwrap();
        let (callback, sender) = $callback;
        let sender_ptr = Box::into_raw(Box::new(sender)) as *mut _;
        let fn_ptr = callback.c_fn();
        $self.register_callback(callback);
        function(manager, $( $args, )* sender_ptr, fn_ptr)
    }};
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
