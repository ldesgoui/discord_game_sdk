use scopeguard::{OnSuccess, ScopeGuard};

type PanicHook = Box<dyn Fn(&std::panic::PanicInfo<'_>) + Sync + Send + 'static>;

// TRACK:
// https://github.com/rust-lang/rust/issues/52652
// https://github.com/rust-lang/rust/issues/58760
// https://github.com/rust-lang/project-ffi-unwind
pub(crate) fn prevent_unwind() -> ScopeGuard<PanicHook, fn(PanicHook), OnSuccess> {
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

    ScopeGuard::with_strategy(hook, |hook| {
        std::panic::set_hook(hook);
    })
}

pub(crate) fn charbuf_to_str(charbuf: &[u8]) -> &str {
    let bytes = &charbuf[..charbuf_len(charbuf)];

    if cfg!(debug_assertions) {
        std::str::from_utf8(bytes).unwrap()
    } else {
        unsafe { std::str::from_utf8_unchecked(bytes) }
    }
}

pub(crate) fn charbuf_len(charbuf: &[u8]) -> usize {
    memchr::memchr(0, charbuf).unwrap_or_else(|| charbuf.len())
}

pub(crate) fn write_charbuf(charbuf: &mut [u8], value: &str) {
    let bytes = value.as_bytes();
    let len = bytes.len();

    debug_assert!(len <= charbuf.len());

    charbuf[..len].copy_from_slice(bytes);

    if len < charbuf.len() {
        charbuf[len] = 0;
    }
}

pub(crate) unsafe fn charptr_to_str<'a>(ptr: *const u8) -> &'a str {
    let bytes = std::ffi::CStr::from_ptr(ptr as *const i8).to_bytes();

    if cfg!(debug_assertions) {
        std::str::from_utf8(bytes).unwrap()
    } else {
        std::str::from_utf8_unchecked(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_charbuf() {
        run_test("");
        run_test("1");
        run_test("10 charact");
        run_test("64 characters 64 characters 64 characters 64 characters 64 chara");
    }

    #[test]
    #[should_panic]
    fn panic_test_write_charbuf() {
        run_test("65 characters 65 characters 65 characters 65 characters 65 charac");
    }

    fn run_test(val: &str) {
        let mut charbuf = [0u8; 64];

        write_charbuf(&mut charbuf, val);

        assert_eq!(charbuf_to_str(&charbuf), val);
    }
}
