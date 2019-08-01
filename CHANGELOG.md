# Change Log

This document contains information about the releases of this crate.

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

