# Change Log

This document contains information about the releases of this crate.

## [Unreleased]

- For `discord_game_sdk`, a `link` feature was added to make the provided linking optional.
- Most `Discord` do not require `&mut self` anymore, only the ones with callbacks remain otherwise.
- The event APIs (`Discord::receivers`, `event::Receivers`) have been replaced with methods returning `Iterator`s to prevent misuse.
- Achievement has been renamed to UserAchievement, some methods had their name updated to reflect this change.
- All enums received a new option, `Undefined(u32)`, to handle cases where a mismatch exists between our API and the SDK.
- All appearances of CStr and CString in the API have been replaced with &str and String for ease of use, no unsafety has been introduced.
- Fixed an integer overflow in `Image::pixel`
- Replaced methods that produced a Vec<T> with their original count+index counterparts, and built an `Iterator` to handle their use easily.
- Doc is two times better
- `Discord::open_lobby_network_channel` now uses `Reliability`, that was missing.
- The difference in terminology in the overlay methods is clearly explained.

## [0.4.2]

- fix Apple OS X regression

## [0.4.1]

- fix `sys/build.rs` for Apple OS X
- fix ActivityKind to support Custom statuses

## [0.4.0]

- `i686-unknown-linux-gnu` and `x86_64-unknown-linux-gnu` are now supported
- `link` feature acts differently, it copies libraries to $OUT_PATH
- ABI broke for Achievements (`percent_complete` is now `u8`, was `i64`)
- dropped dependency to `chrono`, timestamps are now `i64`

## [0.3.1]

- removed incorrect lifetime annotations
- methods who used to take a LobbySearch, LobbyTransaction and LobbyMemberTransaction by value now take it by reference

## [0.3.0]

- dropped `discord_game_sdk_mock`
- reworked features so that they are additive
- added achievement support

## [0.2.5]

- fix SIGILL in `discord_game_sdk_mock`

## [0.2.4]

- `discord_game_sdk_mock` does not use its own logger anymore

## [0.2.3]

- The Last Fix On This, I Promise

## [0.2.2]

- Hotterfix for `discord_game_sdk` and `discord_game_sdk_mock`
    - `discord_game_sdk` will instead use `discord_game_sdk_mock` instead of trying to link to it

## [0.2.1]

- Hotfix for `discord_game_sdk_mock`:
    - Fixed linking errors by forcing a constant output name

## [0.2.0]

- Complete "rewrite" (0.1.0 wasn't really usable in any way).
- Most SDK features have an implemented counterpart except for:
    - Voice InputMethod is not implemented yet.
    - Achievements are not documented and not implemented in this version.
- The SDK is currently in development, this crate is therefore unstable.

