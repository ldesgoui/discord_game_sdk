//! Safe wrapper for the [Discord Game SDK]
//!
//! This crate provides Rust support to the the following Discord features:
//!
//! - Activities (Rich Presence)
//! - Users, Avatars and Relationships
//! - Lobbies, Matchmaking and Voice communication
//! - Faux-P2P Networking
//! - Cloud Synchronized (or not) Storage
//! - Store transactions
//!
//!
//! # Status
//!
//! This library is currently in very early stages, most of the API is implemented but unstable.
//! I'm not aware of any good ways to test this crate.
//!
//!
//! # API stability
//!
//! API stability is completely uncertain until Discord provides details on their update process
//! and how breaking changes will be introduced. The SDK documentations clearly mention that the
//! API is not currently stabilized.
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
//! # "Legal" note
//!
//! This wrapper was informally allowed for publication and distribution by Discord Staff.
//! I cannot redistribute the SDK files until it is made open-source or is licensed for
//! redistribution. You will have to follow some instructions when first setting up your project.
//! Apologies for the inconvenience.
//!
//! If you're a part of Discord and wish to discuss this, please
//! email `ldesgoui@gmail.com` or contact `twiikuu#0047`. I mean no harm.
//!
//!
//! [Discord Game SDK]: https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide

#![doc(html_root_url = "https://docs.rs/discord_game_sdk")]

#[macro_use]
mod macros;

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
///
/// let mut discord = discord_game_sdk::Discord::new(999999999999999999)?;
/// let recvs = discord.event_receivers();
///
/// loop {
///     discord.empty_event_receivers();
///     discord.run_callbacks();
///
///     for _ in recvs.current_user_update.try_iter() {
///         println!("User updated!"),
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
