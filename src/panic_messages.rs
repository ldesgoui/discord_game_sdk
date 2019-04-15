pub(crate) const INVALID_ENUM: &str =
    "[discord_game_sdk] could not match enum value to any definition, this crate might be out of \
     date as the SDK is sending incompatible data";

pub(crate) const NO_NUL: &str =
    "[discord_game_sdk] received a string from the SDK that was not NUL terminated";

pub(crate) const NOT_UTF8: &str =
    "[discord_game_sdk] received a string from the SDK that was not valid UTF-8";
