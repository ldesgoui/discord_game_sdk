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

macro_rules! prevent_unwind {
    () => {
        let hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(|info| {
            log::error!("panic across FFI bounds: {}", info);
            eprintln!(
                "
discord_game_sdk:
    The program has encountered a `panic` across FFI bounds, unwinding at this
    point would be undefined behavior, we will abort the process instead.
    Please report this issue to https://github.com/ldesgoui/discord_game_sdk
    Here is the panic message:

{}
            ",
                info
            );

            std::process::abort();
        }));

        scopeguard::defer_on_success!({
            std::panic::set_hook(hook);
        });
    };
}

macro_rules! get_str {
    ($doc:expr, $name:ident, $($field:tt)+) => {
        #[doc = $doc]
        #[doc = "\n## Cost\n\nString length is calculated every call"]
        pub fn $name(&self) -> &str {
            use crate::utils::cstr_to_str;

            let field = &(self.0).$($field)+;

            cstr_to_str(field)
        }
    };

    ($name:ident, $($field:tt)+) => {
        #[doc = "## Cost\n\nString length is calculated every call"]
        pub fn $name(&self) -> &str {
            use crate::utils::cstr_to_str;

            let field = &(self.0).$($field)+;

            cstr_to_str(field)
        }
    };
}

macro_rules! set_str {
    ($doc:expr, $name:ident, $($field:tt)+) => {
        #[doc = $doc]
        #[doc = "## Panics\n\n`value` must be smaller than the container it is being written to"]
        pub fn $name(&'_ mut self, value: impl AsRef<std::ffi::CStr>) -> &'_ mut Self {
            use crate::utils::slice_u8_to_i8;

            let bytes: &[i8] = slice_u8_to_i8(value.as_ref().to_bytes_with_nul());
            let field = &mut (self.0).$($field)+;

            debug_assert!(bytes.len() <= field.len());

            field[..bytes.len()].copy_from_slice(bytes);

            self
        }
    };

    ($name:ident, $($field:tt)+) => {
        #[doc = "## Panics\n\n`value` must be smaller than the container it is being written to"]
        pub fn $name(&'_ mut self, value: impl AsRef<std::ffi::CStr>) -> &'_ mut Self {
            use crate::utils::slice_u8_to_i8;

            let bytes: &[i8] = slice_u8_to_i8(value.as_ref().to_bytes_with_nul());
            let field = &mut (self.0).$($field)+;

            debug_assert!(bytes.len() <= field.len());

            field[..bytes.len()].copy_from_slice(bytes);

            self
        }
    };
}
