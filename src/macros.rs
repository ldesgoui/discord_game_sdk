macro_rules! ffi {
    // ffi!(self.destroy()) -> Result<()>
    (
        $self:ident
        .
        $method:ident
        (
            $( $args:expr ),*
            $( , )?
        )
    ) => { unsafe {
        use crate::error::{BindingsViolation::*, ToResult};

        match $self.core_ptr.as_ref().ok_or(NullPointer)
            .and_then(|c| c.$method.ok_or(MissingMethod))
        {
            Err(err) => Err(err.into()),
            Ok(method) => method($self.core_ptr, $( $args ),* ).to_result(),
        }
    }};

    // ffi!(self.get_application_manager().get_current_locale()) -> Result<()>
    (
        $self:ident
        .
        $get_manager:ident
        ()
        .
        $method:ident
        (
            $( $args:expr ),*
            $( , )?
        )
    ) => { unsafe {
        use crate::error::{BindingsViolation::*, ToResult};

        match $self.core_ptr.as_ref().ok_or(NullPointer)
            .and_then(|c| c.$get_manager.ok_or(MissingMethod))
            .and_then(|f| f($self.core_ptr).as_mut().ok_or(NullPointer))
        {
            Err(err) => Err(err.into()),
            Ok(mgr) => match mgr.$method {
                None => Err(MissingMethod.into()),
                Some(method) => method(mgr as *mut _, $( $args ),* ).to_result(),
            },
        }
    }};
}
