macro_rules! prevent_unwind {
    () => {
        scopeguard::defer_on_unwind!({
            log::error!("mock library has panicked, aborting");
            std::process::abort();
        });
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
