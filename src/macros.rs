macro_rules! ffi {
    // ffi!(self.destroy())
    ($self:ident . $method:ident ( $($args:expr),* $(,)? )) => {
        {
            debug_assert!($self.core.$method.is_some());

            $self.core.$method.unwrap()($self.core as *mut _, $( $args ),*)
        }
    };

    // ffi!(self.get_application_manager().get_current_locale())
    ($self:ident . $get_manager:ident () . $method:ident ( $($args:expr),* $(,)? )) => {
        {
            let manager = ffi!($self.$get_manager()).as_mut().unwrap();

            debug_assert!(manager.$method.is_some());

            manager.$method.unwrap()(manager as *mut _, $( $args ),*)
        }
    };
}
