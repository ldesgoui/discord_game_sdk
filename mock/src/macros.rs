macro_rules! prevent_unwind {
    () => {
        scopeguard::defer_on_unwind!({
            log::error!(target: "MOCK", "mock library has panicked, aborting");
            std::process::abort();
        });
    };
}

macro_rules! logged_assert {
    ($cond:expr) => {
        if !$cond {
            log::error!(target: "MOCK", "assertion error: {}", stringify!($cond));
            std::process::abort();
        }
    }
}

macro_rules! from_ptr {
    ($name:ident, $typ:path, $($field:tt)+) => {
        unsafe fn $name<'a>(ptr: *mut $typ) -> &'a mut Self {
            &mut *(ptr.sub(memoffset::offset_of!(Self, $($field)+)) as *mut _)
        }
    };
}
