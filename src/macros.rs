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
