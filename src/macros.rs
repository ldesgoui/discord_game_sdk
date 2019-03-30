macro_rules! ffi {
    ($self:ident . $method:ident ( $( $args:expr, )* ) ) => {
        unsafe {
            debug_assert!(!$self.core_ptr.is_null());
            debug_assert!((*$self.core_ptr).$method.is_some());

            (*$self.core_ptr).$method.unwrap()(
                $self.core_ptr,
                $( $args ),*
            )
        }
    };

    ($self:ident . $get_manager:ident () . $method:ident ( $( $args:expr, )* ) ) => {{
        let manager = ffi!($self.$get_manager());

        unsafe {
            debug_assert!(!manager.is_null());
            debug_assert!((*manager).$method.is_some());

            (*manager).$method.unwrap()(
                manager,
                $( $args ),*
            )
        }
    }};

    ($self:ident . $method:ident ( $( $args:expr ),* ) ) => {
        ffi!($self.$method( $( $args ),* ,))
    };

    ($self:ident . $get_manager:ident () . $method:ident ( $( $args:expr ),* ) ) => {
        ffi!($self.$get_manager().$method( $( $args ),* ,))
    };
}
