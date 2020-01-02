pub(crate) const ACROSS_FFI: &str = "[discord_game_sdk]
    The program has encountered a `panic` across FFI bounds, unwinding at this
    point would be undefined behavior, we will abort the process instead.
    Please report this issue to https://github.com/ldesgoui/discord_game_sdk
    Here is the panic message:";

pub(crate) const NULL_PTR: &str =
    "[discord_game_sdk] received a NULL pointer where a valid pointer is expected";
