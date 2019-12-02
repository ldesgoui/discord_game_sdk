# Change Log

This document contains information about the releases of this crate.

## [Unreleased]

- For `discord_game_sdk`, a `link` feature was added to make the provided linking optional.
- All `Discord` methods do not require `&mut self` anymore except for `run_callbacks`.
- Events have been renamed (sorry)
- Event APIs (`Discord::receivers`, `event::Receivers`) have been replaced with methods returning `Iterator`s to prevent misuse.
- Achievement has been renamed to UserAchievement, some methods had their name updated to reflect this change.
- All enums received a new option, `Undefined(u32)`, to handle cases where a mismatch exists between our API and the SDK.
- All appearances of `CStr` and `CString` in the API have been replaced with `&str` and `Cow<str>` for ease of use, no unsafety has been introduced. In the case of `Cow<str>`, it is possible to prevent reallocation by nul-terminating strings manually.
- Fixed an integer overflow in `Image::pixel`.
- Replaced methods that produced a `Vec<T>` with their original `_count` and `_index` method counterparts, as well as methods producing `Iterator`s using these methods
- `Discord::open_lobby_network_channel` now uses `Reliability`, that was missing.
- The difference in terminology in the overlay methods is clearly explained.
- The field in `event::OverlayToggle` has flipped (was opened now is closed)

```sh
sed -i '
    s/achievements::Update/UserAchievementUpdate/g;
    s/activities::Join/ActivityJoin/g;
    s/activities::Spectate/ActivitySpectate/g;
    s/activities::Request/ActivityRequest/g;
    s/activities::Invite/ActivityInvite/g;
    s/lobbies::Update/LobbyUpdate/g;
    s/lobbies::Delete/LobbyDelete/g;
    s/lobbies::MemberConnect/LobbyMemberConnect/g;
    s/lobbies::MemberUpdate/LobbyMemberUpdate/g;
    s/lobbies::MemberDisconnect/LobbyMemberDisconnect/g;
    s/lobbies::Message/LobbyMessage/g;
    s/lobbies::Speaking/LobbySpeaking/g;
    s/lobbies::NetworkMessage/LobbyNetworkMessage/g;
    s/networking::Message/NetworkMessage/g;
    s/networking::RouteUpdate/NetworkRouteUpdate/g;
    s/overlay::Toggle/OverlayToggle/g;
    s/relationships::Refresh/RelationshipsRefresh/g;
    s/relationships::Update/RelationshipUpdate/g;
    s/store::EntitlementCreate/StoreEntitlementCreate/g;
    s/store::EntitlementDelete/StoreEntitlementDelete/g;
    s/users::CurrentUserUpdate/CurrentUserUpdate/g;
    s/voice::SettingsUpdate/VoiceSettingsUpdate/g;
' $(find . -name '*.rs')
```


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

