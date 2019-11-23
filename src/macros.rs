macro_rules! ffi {
    // ffi!(self.destroy())
    (
        $self:ident
        . $method:ident ($($args:expr),* $(,)?)
    ) => {{
        use crate::panic_messages::NULL_PTR;
        log::trace!("FFI: {}", stringify!($method));
        let function = (*$self.core).$method.expect(NULL_PTR);
        function($self.core, $( $args ),*)
    }};

    // ffi!(self.get_application_manager().get_current_locale())
    (
        $self:ident
        . $get_manager:ident ()
        . $method:ident ($($args:expr),* $(,)?)
    ) => {{
        use crate::panic_messages::NULL_PTR;
        let manager = ffi!($self.$get_manager());
        log::trace!("FFI: .{}", stringify!($method));
        let function = (*manager).$method.expect(NULL_PTR);
        function(manager, $( $args ),*)
    }};

    // ffi!(self.get_activity_manager().accept_invite(user_id).and_then(callback))
    (
        $self:ident
        . $get_manager:ident ()
        . $method:ident ($($args:expr),* $(,)?)
        . and_then ($callback:expr $(,)?)
    ) => {{
        use crate::panic_messages::NULL_PTR;
        let manager = ffi!($self.$get_manager());
        log::trace!("FFI: .{}", stringify!($method));
        let function = (*manager).$method.expect(NULL_PTR);
        let (callback, sender) = $callback;
        let sender_ptr = Box::into_raw(Box::new(sender)) as *mut _;
        let fn_ptr = callback.c_fn();
        $self.register_callback(callback);
        function(manager, $( $args, )* sender_ptr, fn_ptr)
    }};
}

// https://github.com/rust-lang/project-ffi-unwind
macro_rules! prevent_unwind {
    () => {
        use crate::panic_messages::ACROSS_FFI;

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
