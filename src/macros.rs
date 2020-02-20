macro_rules! with_manager {
    ($with_manager: ident, $get_manager: ident, $type: ty) => {
        pub(crate) fn $with_manager<T>(&self, callback: impl FnOnce(&mut $type) -> T) -> T {
            self.with_core(|core| {
                let ptr = unsafe { core.$get_manager.unwrap()(core) };
                utils::with_tx(ptr, callback)
            })
        }
    };
}

// TRACK:
// https://github.com/rust-lang/rust/issues/52652
// https://github.com/rust-lang/rust/issues/58760
// https://github.com/rust-lang/project-ffi-unwind
macro_rules! prevent_unwind {
    () => {
        const ACROSS_FFI: &str = "[discord_game_sdk]
            The program has encountered a `panic` across FFI bounds, unwinding at this
            point would be undefined behavior, we will abort the process instead.
            Please report this issue to https://github.com/ldesgoui/discord_game_sdk
            Here is the panic message:";

        let hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(|info| {
            log::error!("panic across FFI bounds: {}", info);
            eprintln!("\n{}\n\n{}\n", ACROSS_FFI, info);
            std::process::abort();
        }));

        scopeguard::defer_on_success!({
            std::panic::set_hook(hook);
        });
    };
}
