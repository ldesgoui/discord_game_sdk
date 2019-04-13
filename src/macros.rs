macro_rules! ffi {
    // ffi!(self.destroy())
    ($self:ident . $method:ident ($($args:expr),* $(,)?)) => {{
        log::trace!("FFI: {}", stringify!($method));
        let function = (*$self.core).$method.unwrap();
        function($self.core, $( $args ),*)
    }};

    // ffi!(self.get_application_manager().get_current_locale())
    ($self:ident .  $get_manager:ident () .  $method:ident ($($args:expr),* $(,)?)) => {{
        let manager = ffi!($self.$get_manager());
        log::trace!("FFI:     {}", stringify!($method));
        let function = (*manager).$method.unwrap();
        function(manager, $( $args ),*)
    }};

    // ffi!(self.get_activity_manager().accept_invite(user_id)(callback))
    ($self:ident .  $get_manager:ident () .  $method:ident ($($args:expr),* $(,)?) ($callback:expr $(,)?)) => {{
        let manager = ffi!($self.$get_manager());
        log::trace!("FFI:     {}", stringify!($method));
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

macro_rules! get_str {
    ($name:ident, $($field:tt)+) => {
        pub fn $name(&self) -> &str {
            std::ffi::CStr::from_bytes_with_nul(unsafe { std::mem::transmute(&(self.0).$($field)+[..]) })
                .unwrap()
                .to_str()
                .unwrap()
        }
    }
}

macro_rules! set_str {
    ($name:ident, $($field:tt)+) => {
        pub fn $name<'a>(&'a mut self, value: impl AsRef<std::ffi::CStr>) -> &'a mut Self {
            let bytes: &[i8] = unsafe { std::mem::transmute(value.as_ref().to_bytes_with_nul()) };
            let field = &mut (self.0).$($field)+;
            debug_assert!(bytes.len() <= field.len());
            field[..bytes.len()].copy_from_slice(bytes);
            self
        }
    }
}
