# discord_game_sdk

[![Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/discord_game_sdk)
[![Latest Version](https://img.shields.io/crates/v/discord_game_sdk.svg)](https://crates.io/crates/discord_game_sdk)
![License](https://img.shields.io/crates/l/discord_game_sdk)
[![Build Status](https://img.shields.io/github/workflow/status/ldesgoui/discord_game_sdk/Continuous%20Integration)](https://github.com/ldesgoui/discord_game_sdk/actions)

This crate provides a safe interface to the [Discord Game SDK].

*This crate is not official, it is not supported by the Discord Game SDK Developers.*

The [Discord Game SDK] provides features such as, but not limited to:

- Activities (Rich Presence)
- Users, Avatars and Relationships
- Lobbies, Matchmaking and Voice communication
- Faux-P2P Networking on Discord's Infrastructure
- Cloud Synchronized Storage
- Store Transactions
- Achievements

*Version requirement: Rust 1.37 and up.*

*[Release Notes](https://github.com/ldesgoui/discord_game_sdk/releases)*


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
discord_game_sdk = "0.999.0"
```

Read up on potential [`bindgen` requirements].

Download the [Discord Game SDK] and set the following environment variable to where you extracted it:

```sh
export DISCORD_GAME_SDK_PATH=/path/to/discord_game_sdk
```


## Features:

- `link`: (enabled by default, delegates to `discord_game_sdk_sys/link`)
    Provides the linker with a copy of the dynamic library.
    This allows for `cargo run` to run with no additional setup on Linux and Windows.

- [`image`](https://docs.rs/image): (optional crate)
    Provides a conversion from our `Image` to `image::RgbaImage`.


## Safety

This crate relies on the SDK to provide correct data and behavior:

- Non-null pointers to valid memory
- UTF-8, NUL-terminated strings
- No mutation of memory it should have no ownership of
- No use of pointers after `destroy` is called

Some of these are tested when compiled with `debug_assertions`.


## Legal

You *MUST* acquaint yourself with and agree to the [official terms of the Discord Game SDK].

The code of the Rust crates `discord_game_sdk` and `discord_game_sdk_sys`
are licensed at your option under either of:

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
* [MIT License](https://opensource.org/licenses/MIT)

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.


## Communication and Support

I can be reached via Discord `twiikuu#0047`, on the [Official Game SDK Server]
(nicked as `ldesgoui (rust wrapper)`), as well as [twitter] and [email].
I reserve myself no obligation to support you, although I'm generally nice.


[Discord Game SDK]: https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide
[Official Game SDK Server]: https://discord.gg/discord-gamesdk
[`bindgen` requirements]: https://rust-lang.github.io/rust-bindgen/requirements.html
[email]: mailto:ldesgoui@ldesgoui.xyz
[official terms of the Discord Game SDK]: https://discordapp.com/developers/docs/legal
[twitter]: https://twitter.com/ldesgoui
