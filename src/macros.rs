macro_rules! ffi {
    // ffi!(self.get_application_manager().get_current_locale()) -> Result<()>
    ($self:ident . $method:ident ( $( $args:expr, )* ) ) => { unsafe {
        || -> Result<()> {
            let core = $self.core_ptr.as_ref().ok_or(Error::NullResult)?;
            let method = core.$method.ok_or(Error::MissingMethod)?;
            method($self.core_ptr, $( $args ),*).to_result()
        }()
    }};

    // ffi!(self.get_application_manager().get_current_locale()) -> Result<()>
    ($self:ident . $get_manager:ident () . $method:ident ( $( $args:expr, )* ) ) => { unsafe {
        || -> Result<()> {
            let core = $self.core_ptr.as_ref().ok_or(Error::NullResult)?;
            let get_manager = core.$get_manager.ok_or(Error::MissingMethod)?;
            let manager_ptr = get_manager($self.core_ptr);
            let manager = manager_ptr.as_ref().ok_or(Error::NullResult)?;
            let method = manager.$method.ok_or(Error::MissingMethod)?;
            method(manager_ptr, $( $args ),*).to_result()
        }()
    }};

    ($self:ident . $method:ident ( $( $args:expr ),* ) ) => {
        ffi!($self.$method( $( $args ),* ,))
    };

    ($self:ident . $get_manager:ident () . $method:ident ( $( $args:expr ),* ) ) => {
        ffi!($self.$get_manager().$method( $( $args ),* ,))
    };
}
