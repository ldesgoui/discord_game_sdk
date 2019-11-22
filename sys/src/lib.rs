//! Rust low-level bindings for the [Discord Game SDK]
//!
//! *This crate is not official, it is not supported by the Discord Game SDK Developers.*
//!
//! Following the `-sys` package conventions, this crate does not define higher-level abstractions.
//!
//!
//! <https://docs.rs/discord_game_sdk_sys>
//!
//! <https://crates.io/crates/discord_game_sdk_sys>
//!
//!
//! # Requirements
//!
//! - https://rust-lang.github.io/rust-bindgen/requirements.html
//! - https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide
//!
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! discord_game_sdk_sys = "0.4.2"
//! ```
//!
//! Set the following environment variable:
//!
//! ```sh
//! export DISCORD_GAME_SDK_PATH=/path/to/discord_game_sdk
//! ```
//!
//! Ensure that appropriately named dynamic libraries are available when launching the program:
//!
//! On Linux and OS X, the dynamic library must be renamed to `libdiscord_game_sdk.so` or
//! `libdiscord_game_sdk.dylib`.
//!
//! (TODO: ^ might be unnecessary, how to set the rpath in the output binary)
//!
//!
//! # Features:
//!
//! - `link`:
//!     Provides the linker with an appropriately named dynamic library.
//!     This allows for `cargo run` to run flawlessly on Linux.
//!
//!
//! # Legal
//!
//! You *MUST* acquaint yourself with and agree to the [official terms of the Discord Game SDK].
//!
//! The code of the Rust crates `discord_game_sdk` and `discord_game_sdk_sys` are licensed under
//! either of:
//!
//! * [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
//! * [MIT License](https://opensource.org/licenses/MIT)
//!
//! at your option.
//!
//!
//! # Communication and Support
//!
//! I can be reached via Discord `twiikuu#0047`, on the [Official Game SDK Server] (nicked as
//! `ldesgoui (rust wrapper)`, as well as [twitter] and [email].
//! I reserve myself no obligation to support you, although I'm generally nice.
//!
//!
//! [Discord Game SDK]: https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide
//! [official terms of the Discord Game SDK]: https://discordapp.com/developers/docs/legal
//! [Official Game SDK Server]: https://discord.gg/discord-gamesdk
//! [twitter]: https://twitter.com/ldesgoui
//! [email]: mailto:ldesgoui@ldesgoui.xyz

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![doc(html_root_url = "https://docs.rs/discord_game_sdk_sys")]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// bindgen knows how to automatically implement PartialEq when it can't be derived but it won't
// automatically implement Eq
impl Eq for DiscordActivity {}
impl Eq for DiscordActivityAssets {}
impl Eq for DiscordActivityParty {}
impl Eq for DiscordActivitySecrets {}
impl Eq for DiscordFileStat {}
impl Eq for DiscordInputMode {}
impl Eq for DiscordLobby {}
impl Eq for DiscordOAuth2Token {}
impl Eq for DiscordPresence {}
impl Eq for DiscordRelationship {}
impl Eq for DiscordSku {}
impl Eq for DiscordUser {}
impl Eq for DiscordUserAchievement {}
