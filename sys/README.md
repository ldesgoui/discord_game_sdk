# discord_game_sdk_sys

Rust low-level bindings for the [Discord Game SDK]

*This crate is not official, it is not supported by the Discord Game SDK Developers.*

Following the `-sys` package conventions, this crate does not define higher-level abstractions.


<https://docs.rs/discord_game_sdk_sys>

<https://crates.io/crates/discord_game_sdk_sys>


## Requirements

- <https://rust-lang.github.io/rust-bindgen/requirements.html>
- <https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide>


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
discord_game_sdk_sys = "0.5.0-alpha.2"
```

Set the following environment variable:

```sh
export DISCORD_GAME_SDK_PATH=/path/to/discord_game_sdk
```


## Features:

- `link`: (enabled by default)
    Provides the linker with a copy of the dynamic library.
    This allows for `cargo run` to run flawlessly on Linux.


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
[official terms of the Discord Game SDK]: https://discordapp.com/developers/docs/legal
[Official Game SDK Server]: https://discord.gg/discord-gamesdk
[twitter]: https://twitter.com/ldesgoui
[email]: mailto:ldesgoui@ldesgoui.xyz
