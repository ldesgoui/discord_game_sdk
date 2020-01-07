//! Safe wrapper for the [Discord Game SDK]
//!
//! *This crate is not official, it is not supported by the Discord Game SDK Developers.*
//!
//! This crate provides Rust support to the following Discord features:
//!
//! - Activities (Rich Presence)
//! - Users, Avatars and Relationships
//! - Lobbies, Matchmaking and Voice communication
//! - Faux-P2P Networking
//! - Cloud Synchronized (or not) Storage
//! - Store transactions
//! - Achievements
//! - ...
//!
//!
//! <https://docs.rs/discord_game_sdk>
//!
//! <https://crates.io/crates/discord_game_sdk>
//!
//!
//! # Requirements
//!
//! - <https://rust-lang.github.io/rust-bindgen/requirements.html>
//! - <https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide>
//!
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! discord_game_sdk = "0.5.0-alpha.5"
//! ```
//!
//! Set the following environment variable:
//!
//! ```sh
//! export DISCORD_GAME_SDK_PATH=/path/to/discord_game_sdk
//! ```
//!
//!
//! # Status
//!
//! This crate is getting quite stable. It should be usable.
//! Testing is quite limited due to the nature of the bindings.
//!
//!
//! # Minimum Supported Rust Version
//!
//! This crate is guaranteed compile on stable Rust.
//! This crate currently compiles on stable Rust 1.37.0,
//! this is tested in CI, just in case we break it.
//!
//!
//! # Features:
//!
//! - `link`: (enabled by default, delegates to `discord_game_sdk_sys/link`)  
//!     Provides the linker with a copy of the dynamic library.  
//!     This allows for `cargo run` to run with no additional setup on Linux and Windows.
//!
//! - [`image`](https://docs.rs/image): (optional crate)  
//!     Provides a conversion between our image buffer and `image::RgbaImage`.
//!
//!
//! # Safety
//!
//! This crate relies on the SDK to provide correct data and behavior:
//! - Non-null pointers to valid memory
//! - UTF-8, NUL-terminated strings
//! - No mutation of memory it should have no ownership of
//! - No use of pointers after `destroy` is called
//!
//! Some of these are tested when compiled with `debug_assertions`.
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
//! # Communication and Support
//!
//! I can be reached via Discord `twiikuu#0047`, on the [Official Game SDK Server]
//! (nicked as `ldesgoui (rust wrapper)`), as well as [twitter] and [email].
//! I reserve myself no obligation to support you, although I'm generally nice.
//!
//!
//! [Discord Game SDK]: https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide
//! [official terms of the Discord Game SDK]: https://discordapp.com/developers/docs/legal
//! [Official Game SDK Server]: https://discord.gg/discord-gamesdk
//! [twitter]: https://twitter.com/ldesgoui
//! [email]: mailto:ldesgoui@ldesgoui.xyz

#![doc(html_root_url = "https://docs.rs/discord_game_sdk/0.5.0-alpha.5")]

#[macro_use]
mod macros;

mod action;
mod activity;
mod activity_kind;
mod aliases;
mod cast;
mod comparison;
mod create_flags;
mod discord;
mod distance;
mod entitlement;
mod entitlement_kind;
mod error;
mod event_handler;
mod fetch_kind;
mod file_stat;
mod image;
mod image_handle;
mod image_kind;
mod input_mode;
mod input_mode_kind;
mod iter;
mod lobby;
mod lobby_kind;
mod lobby_member_transaction;
mod lobby_transaction;
mod log_level;
mod oauth2_token;
pub(crate) mod panic_messages;
mod premium_kind;
mod presence;
mod relationship;
mod relationship_kind;
mod reliability;
mod request_reply;
mod search_query;
mod sku;
mod sku_kind;
mod status;
mod to_result;
mod user;
mod user_achievement;
mod user_flags;
pub(crate) mod utils;

mod methods {
    mod core;

    mod achievements;
    mod activities;
    mod applications;
    mod images;
    mod lobbies;
    mod networking;
    mod overlay;
    mod relationships;
    mod storage;
    mod store;
    mod users;
    mod voice;
}

pub(crate) mod across_ffi {
    pub(crate) mod event_handlers;
}

pub(crate) use discord_game_sdk_sys as sys;

pub use self::{
    action::Action,
    activity::Activity,
    activity_kind::ActivityKind,
    aliases::*,
    cast::Cast,
    comparison::Comparison,
    create_flags::CreateFlags,
    discord::Discord,
    distance::Distance,
    entitlement::Entitlement,
    entitlement_kind::EntitlementKind,
    error::{Error, Result},
    event_handler::EventHandler,
    fetch_kind::FetchKind,
    file_stat::FileStat,
    image::Image,
    image_handle::ImageHandle,
    image_kind::ImageKind,
    input_mode::InputMode,
    input_mode_kind::InputModeKind,
    iter::Collection,
    lobby::Lobby,
    lobby_kind::LobbyKind,
    lobby_member_transaction::LobbyMemberTransaction,
    lobby_transaction::LobbyTransaction,
    log_level::LogLevel,
    oauth2_token::OAuth2Token,
    premium_kind::PremiumKind,
    presence::Presence,
    relationship::Relationship,
    relationship_kind::RelationshipKind,
    reliability::Reliability,
    request_reply::RequestReply,
    search_query::SearchQuery,
    sku::Sku,
    sku_kind::SkuKind,
    status::Status,
    user::User,
    user_achievement::UserAchievement,
    user_flags::UserFlags,
};
