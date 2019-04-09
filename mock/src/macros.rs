macro_rules! prevent_unwind {
    () => {
        // scopeguard::defer_on_unwind!({
        //     log::error!(target: "MOCK", "mock library has panicked, aborting");
        //     std::process::abort();
        // });
    };
}

macro_rules! logged_assert {
    ($cond:expr) => {
        if !$cond {
            log::error!("assertion error: {}", stringify!($cond));
            std::process::abort();
        }
    };
}

macro_rules! from_ptr {
    ($name:ident, $typ:path, $($field:tt)+) => {
        pub unsafe fn $name<'a>(ptr: *mut $typ) -> &'a mut Self {
            let offset = memoffset::offset_of!(Self, $($field)+);
            let ptr = ptr as *const u8;
            let ptr = ptr.sub(offset);
            let ptr = ptr as *mut Self;
            ptr.as_mut().unwrap()
        }
    };
}
