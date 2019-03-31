macro_rules! ffi {
    ($self:ident . $method:ident ( $( $args:expr, )* ) ) => {
        unsafe {
            if $self.core_ptr.is_null() {
                Err($crate::error::Error::NullResult)?;
            }
            let func = (*$self.core_ptr).$method.ok_or($crate::error::Error::MissingMethod)?;
            func($self.core_ptr, $( $args ),*)
        }
    };

    ($self:ident . $get_manager:ident () . $method:ident ( $( $args:expr, )* ) ) => {{
        let manager = ffi!($self.$get_manager());

        unsafe {
            if manager.is_null() {
                Err($crate::error::Error::NullResult)?
            }
            let func = (*manager).$method.ok_or($crate::error::Error::MissingMethod)?;
            func(manager, $( $args ),*)
        }
    }};

    ($self:ident . $method:ident ( $( $args:expr ),* ) ) => {
        ffi!($self.$method( $( $args ),* ,))
    };

    ($self:ident . $get_manager:ident () . $method:ident ( $( $args:expr ),* ) ) => {
        ffi!($self.$get_manager().$method( $( $args ),* ,))
    };

    ($self:ident . $method:ident ( $( $args:expr, )* ) ? ) => {
        $crate::error::discord_result(ffi!($self.$method( $( $args, )*)))?
    };

    ($self:ident . $get_manager:ident () . $method:ident ( $( $args:expr, )* ) ? ) => {
        $crate::error::discord_result(ffi!($self.$get_manager().$method( $( $args, )*)))?
    };

    ($self:ident . $method:ident ( $( $args:expr ),* ) ? ) => {
        $crate::error::discord_result(ffi!($self.$method( $( $args ),*)))?
    };

    ($self:ident . $get_manager:ident () . $method:ident ( $( $args:expr ),* ) ? ) => {
        $crate::error::discord_result(ffi!($self.$get_manager().$method( $( $args ),*)))?
    };
}
