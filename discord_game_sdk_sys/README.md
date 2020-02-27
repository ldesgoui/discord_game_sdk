# discord_game_sdk_sys

[![Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/discord_game_sdk_sys)
[![Latest Version](https://img.shields.io/crates/v/discord_game_sdk_sys.svg)](https://crates.io/crates/discord_game_sdk_sys)
![License](https://img.shields.io/crates/l/discord_game_sdk_sys)

This crate provides `bindgen`-generated bindings to the [Discord Game SDK].

*This crate is not official, it is not supported by the Discord Game SDK Developers.*

Following the `-sys` package conventions, this crate does not define higher-level abstractions.


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
discord_game_sdk_sys = "1.0.0-rc.1"
```

Read up on potential [`bindgen` requirements].

Download the [Discord Game SDK] and set the following environment variable to where you extracted it:

```sh
export DISCORD_GAME_SDK_PATH=/path/to/discord_game_sdk
```

If you're also planning on using the default `link` feature, keep reading below.


## Features:

#### `link`

Enabled by default, delegates to `discord_game_sdk_sys/link`.

Provides functional linking with the caveat that libraries are renamed and some additional
set-up is required:

```sh
# Linux: prepend with `lib`
cp $DISCORD_GAME_SDK_PATH/lib/x86_64/{,lib}discord_game_sdk.so

# Mac OS: prepend with `lib` and add to library search path
cp $DISCORD_GAME_SDK_PATH/lib/x86_64/{,lib}discord_game_sdk.dylib
export DYLD_LIBRARY_PATH=$DYLD_LIBRARY_PATH:$DISCORD_GAME_SDK_PATH/lib/x86_64

# Windows: change `dll.lib` to `lib` (won't affect library search)
cp $DISCORD_GAME_SDK_PATH/lib/x86_64/discord_game_sdk.{dll.lib,lib}
cp $DISCORD_GAME_SDK_PATH/lib/x86/discord_game_sdk.{dll.lib,lib}
```

This allows for `cargo run` to function.


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
