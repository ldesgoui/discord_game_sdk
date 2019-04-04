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

#![allow(unused_variables, unused_imports)]

// This absolutely needs to come first
#[macro_use]
mod macros;

mod activity;
mod discord;
mod entitlement;
pub mod error;
pub mod event;
mod file;
mod oauth2_token;
mod relationship;
mod sku;
mod user;
mod utils;

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

mod across_ffi {
    pub(crate) mod activity;
    pub(crate) mod callbacks;
    pub(crate) mod lobby;
    pub(crate) mod network;
    pub(crate) mod overlay;
    pub(crate) mod relationship;
    pub(crate) mod store;
    pub(crate) mod user;
    pub(crate) mod voice;
}

mod prelude {
    pub(crate) use crate::{
        across_ffi,
        error::{DiscordError, ToResult as _},
        utils::{string_from_cstr, FromSys},
        *,
    };
    pub(crate) use discord_game_sdk_sys as sys;
    pub(crate) use std::ffi::{CStr, CString};
    pub(crate) use std::os::raw::{c_char, c_void};
}

pub use crate::{
    activity::{Action, Activity, ActivityChange, ActivityKind, RequestReply},
    discord::{CreateFlags, Discord},
    entitlement::{Entitlement, EntitlementKind},
    error::{Error, Result},
    file::FileStat,
    oauth2_token::OAuth2Token,
    relationship::{Presence, Relationship, RelationshipKind, Status},
    sku::{Sku, SkuKind},
    user::{PremiumType, User},
};
