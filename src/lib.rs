//! Safe wrapper for the [Discord Game SDK](https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide).
//!
//! # Status
//!
//! This library is currently in very early stages, most of the API is missing.
//!
//! # "Legal" note
//!
//! This wrapper was informally allowed for publication and distribution by Discord Staff.
//! I cannot redistribute the SDK files until it is made open-source or is licensed for redistribution. You will have to follow some instructions when first setting up your project.
//! This also means that docs.rs will not be able to build the documentation.
//! Apologies for the inconvenience.
//!
//! If you're a part of Discord and wish to discuss this, please email `ldesgoui@gmail.com` or contact `twiikuu#0047`. I mean no harm.

#![recursion_limit = "128"]

#[cfg(feature = "mock")]
#[link(name = "discord_game_sdk_mock")]
extern "C" {}

#[macro_use]
mod macros;

pub(crate) use discord_game_sdk_sys as sys;

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
    error::{DiscordError, DiscordResult},
    fetch_kind::FetchKind,
    file_stat::FileStat,
    image::Image,
    image_handle::ImageHandle,
    image_kind::ImageKind,
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
