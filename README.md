Safe wrapper for the [Discord Game SDK]

*This crate is not official, it is not supported by the Discord Game SDK Developers.*

This crate provides Rust support to the following Discord features:

- Activities (Rich Presence)
- Users, Avatars and Relationships
- Lobbies, Matchmaking and Voice communication
- Faux-P2P Networking
- Cloud Synchronized (or not) Storage
- Store transactions
- Achievements
- ...


<https://docs.rs/discord_game_sdk>

<https://crates.io/crates/discord_game_sdk>


# Requirements

- <https://rust-lang.github.io/rust-bindgen/requirements.html>
- <https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide>


# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
discord_game_sdk = "0.5.0-alpha.1"
```

Set the following environment variable:

```sh
export DISCORD_GAME_SDK_PATH=/path/to/discord_game_sdk
```

Ensure that appropriately named dynamic libraries are available when launching the program:

On Linux and OS X, the dynamic library must be renamed to `libdiscord_game_sdk.so` or
`libdiscord_game_sdk.dylib`.

(TODO: ^ might be unnecessary, how to set the rpath in the output binary)


# Status

This library is currently in early stages but is stabilizing. It is usable.
I'm not aware of any good ways to test this crate, there are currently no tests.


# Features:

- `link`: (enabled by default, delegates to `discord_game_sdk_sys/link`)
    Provides the linker with an appropriately named dynamic library.
    This allows for `cargo run` to run flawlessly on Linux.


# Safety

This crate relies on the SDK to provide correct data and behavior:
- Non-null pointers to valid memory
- UTF-8, NUL-terminated strings
- No mutation of memory it should have no ownership of
- No use of pointers after `destroy` is called


# Legal

You *MUST* acquaint yourself with and agree to the [official terms of the Discord Game SDK].

The code of the Rust crates `discord_game_sdk` and `discord_game_sdk_sys` are licensed under
either of:

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
* [MIT License](https://opensource.org/licenses/MIT)

at your option.


# Communication and Support

I can be reached via Discord `twiikuu#0047`, on the [Official Game SDK Server] (nicked as
`ldesgoui (rust wrapper)`), as well as [twitter] and [email].
I reserve myself no obligation to support you, although I'm generally nice.


[Discord Game SDK]: https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide
[official terms of the Discord Game SDK]: https://discordapp.com/developers/docs/legal
[Official Game SDK Server]: https://discord.gg/discord-gamesdk
[twitter]: https://twitter.com/ldesgoui
[email]: mailto:ldesgoui@ldesgoui.xyz
