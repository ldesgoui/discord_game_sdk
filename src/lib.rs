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
//! discord_game_sdk = "0.4.1"
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
//! # Status
//!
//! This library is currently in early stages but is stabilizing. It is usable.
//! I'm not aware of any good ways to test this crate, there are currently no tests.
//!
//!
//! # Features:
//!
//! - `link`: (enabled by default, delegates to `discord_game_sdk_sys/link`)
//!     Provides the linker with an appropriately named dynamic library.
//!     This allows for `cargo run` to run flawlessly on Linux.
//!
//!
//! # Safety
//!
//! This crate relies on the SDK to provide correct data and behavior:
//! - Non-null pointers to valid memory
//! - UTF-8, NUL-terminated strings
//! - Valid enum values
//! - No mutation of memory it should have no ownership of
//! - No use of pointers after `destroy` is called
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

#![doc(html_root_url = "https://docs.rs/discord_game_sdk")]

#[macro_use]
mod macros;

mod achievement;
mod action;
mod activity;
mod activity_kind;
mod callbacks;
mod cast;
mod comparison;
mod create_flags;
mod discord;
mod distance;
mod entitlement;
mod entitlement_kind;
mod error;
mod fetch_kind;
mod file_stat;
mod image;
mod image_handle;
mod image_kind;
mod input_mode;
mod input_mode_kind;
mod lobby;
mod lobby_kind;
mod lobby_member_transaction;
mod lobby_transaction;
pub(crate) mod macro_helper;
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

/// # Event Types
///
/// This crate makes use of [`crossbeam_channel`] to pass events.
///
/// All event and callback handlers do a minimal amount of work possible across FFI; they send
/// copied or cloned data. Here is why:
/// - Panics in FFI must be intercepted and unwinding must be disabled
/// - Passing a mutable reference to [`Discord`] would result in either mutable aliasing or
///   deadlocks (if using Arc<Mutex<_>>)
///
/// If an event or callback handler runs into a panic across FFI, the panic will be
/// intercepted and the process will be aborted.
///
///
/// ### IMPORTANT NOTE:
/// If you do not make use of all receivers, you must call [`Discord::empty_event_receivers`]
/// or [`event::Receivers::empty_channels`] to prevent the event buffers from growing
/// too big. A safe place to do that would be just before [`Discord::run_callbacks`].
///
///
/// ### IMPORTANT NOTE:
/// Unless you plan to run [`Discord::run_callbacks`] in another thread, waiting for events
/// will block forever. This means that `try_` methods should be used and `select!` must
/// contain a `default` clause.
///
///
/// ### IMPORTANT NOTE:
/// [`crossbeam_channel::Receiver`]s (and [`event::Receivers`]) may be cloned but the events
/// won't be duplicated. An event may only be received once across the whole application.
/// Dispatching events to multiple different places is on you.
///
///
/// ## Examples
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut discord = discord_game_sdk::Discord::new(999999999999999999)?;
///
/// loop {
///     discord.empty_event_receivers();
///     discord.run_callbacks()?;
///
///     let recvs = discord.event_receivers();
///     for _ in recvs.current_user_update.try_iter() {
///         println!("User updated!");
///     }
/// }
/// # Ok(()) }
/// ```
///
/// [`Discord::empty_event_receivers`]: ../struct.Discord.html#method.empty_event_receivers
/// [`Discord::run_callbacks`]: ../struct.Discord.html#method.run_callbacks
/// [`Discord`]: ../struct.Discord.html
/// [`crossbeam_channel::Receiver`]: https://docs.rs/crossbeam-channel/latest/crossbeam_channel/struct.Receiver.html
/// [`crossbeam_channel::Sender`]: https://docs.rs/crossbeam-channel/latest/crossbeam_channel/struct.Sender.html
/// [`crossbeam_channel`]: https://docs.rs/crossbeam-channel
/// [`event::Receivers::empty_channel`]: struct.Receivers.html#method.empty_channels
/// [`event::Receivers`]: struct.Receivers.html
pub mod event {
    pub mod achievements;
    pub mod activities;
    mod channels;
    pub mod lobbies;
    pub mod networking;
    pub mod overlay;
    pub mod relationships;
    pub mod store;
    pub mod users;
    pub mod voice;

    pub use self::channels::Receivers;
    pub(crate) use self::channels::*;
}

pub(crate) mod across_ffi {
    pub(crate) mod achievements;
    pub(crate) mod activities;
    pub(crate) mod callbacks;
    pub(crate) mod lobbies;
    pub(crate) mod networking;
    pub(crate) mod overlay;
    pub(crate) mod relationships;
    pub(crate) mod store;
    pub(crate) mod users;
    pub(crate) mod voice;
}

pub(crate) use discord_game_sdk_sys as sys;

pub use self::{
    achievement::Achievement,
    action::Action,
    activity::Activity,
    activity_kind::ActivityKind,
    cast::Cast,
    comparison::Comparison,
    create_flags::CreateFlags,
    discord::Discord,
    distance::Distance,
    entitlement::Entitlement,
    entitlement_kind::EntitlementKind,
    error::{Error, Result},
    fetch_kind::FetchKind,
    file_stat::FileStat,
    image::Image,
    image_handle::ImageHandle,
    image_kind::ImageKind,
    input_mode::InputMode,
    input_mode_kind::InputModeKind,
    lobby::Lobby,
    lobby_kind::LobbyKind,
    lobby_member_transaction::LobbyMemberTransaction,
    lobby_transaction::LobbyTransaction,
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
    user_flags::UserFlags,
};
