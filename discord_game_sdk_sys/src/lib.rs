//! This crate provides `bindgen`-generated bindings to the [Discord Game SDK].
//!
//! *This crate is not official, it is not supported by the Discord Game SDK Developers.*
//!
//! Following the `-sys` package conventions, this crate does not define higher-level abstractions.
//!
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! discord_game_sdk_sys = "1.0.0"
//! ```
//!
//! Read up on potential [`bindgen` requirements].
//!
//! Download the [Discord Game SDK] and set the following environment variable to where you extracted it:
//!
//! ```sh
//! export DISCORD_GAME_SDK_PATH=/path/to/discord_game_sdk
//! ```
//!
//! If you're also planning on using the default `link` feature, keep reading below.
//!
//!
//! # Features:
//!
//! ### `link`
//!
//! Enabled by default, delegates to `discord_game_sdk_sys/link`.
//!
//! Provides functional linking with the caveat that libraries are renamed and some additional
//! set-up is required:
//!
//! ```sh
//! # Linux: prepend with `lib` and add to library search path
//! cp $DISCORD_GAME_SDK_PATH/lib/x86_64/{,lib}discord_game_sdk.so
//! export LD_LIBRARY_PATH=${LD_LIBRARY_PATH:+${LD_LIBRARY_PATH}:}$DISCORD_GAME_SDK_PATH/lib/x86_64
//!
//! # Mac OS: prepend with `lib` and add to library search path
//! cp $DISCORD_GAME_SDK_PATH/lib/x86_64/{,lib}discord_game_sdk.dylib
//! export DYLD_LIBRARY_PATH=${DYLD_LIBRARY_PATH:+${DYLD_LIBRARY_PATH}:}$DISCORD_GAME_SDK_PATH/lib/x86_64
//!
//! # Windows: change `dll.lib` to `lib` (won't affect library search)
//! cp $DISCORD_GAME_SDK_PATH/lib/x86_64/discord_game_sdk.{dll.lib,lib}
//! cp $DISCORD_GAME_SDK_PATH/lib/x86/discord_game_sdk.{dll.lib,lib}
//! ```
//!
//! This allows for `cargo run` to function.
//!
//!
//! # Legal
//!
//! You *MUST* acquaint yourself with and agree to the [official terms of the Discord Game SDK].
//!
//! The code of the Rust crates `discord_game_sdk` and `discord_game_sdk_sys`
//! are licensed at your option under either of:
//!
//! * [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
//! * [MIT License](https://opensource.org/licenses/MIT)
//!
//! Unless you explicitly state otherwise, any contribution intentionally
//! submitted for inclusion in the work by you, as defined in the Apache-2.0
//! license, shall be dual licensed as above, without any additional terms or
//! conditions.
//!
//!
//! [Discord Game SDK]: https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide
//! [`bindgen` requirements]: https://rust-lang.github.io/rust-bindgen/requirements.html
//! [official terms of the Discord Game SDK]: https://discordapp.com/developers/docs/legal

#![allow(clippy::all, warnings)]
#![doc(html_root_url = "https://docs.rs/discord_game_sdk_sys/1.0.0")]

// Strings in the SDK are already UTF-8, we rarely end up using CStr/CString because of the
// development overhead costs, `u8`s mean we have to do less conversions in code.
pub(crate) mod ctypes {
    pub(crate) use std::os::raw::{c_uchar as c_char, *};
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
