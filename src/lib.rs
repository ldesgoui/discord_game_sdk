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

#[macro_use]
mod macros;

mod activity;
mod discord;
mod entitlement;
pub mod error;
mod file;
mod lobby;
mod lobby_transaction;
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

pub mod event {
    pub mod activities;
    pub mod channels;
    pub mod lobbies;
    pub mod networking;
    pub mod overlay;
    pub mod relationships;
    pub mod store;
    pub mod users;
    pub mod voice;

    pub use self::channels::Receivers;
    pub(crate) use self::channels::{create_channels, Senders};
}

mod across_ffi {
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

mod prelude {
    #[allow(unused_imports)]
    pub(crate) use crate::error::ToResult;
    pub(crate) use crate::{
        across_ffi::{self, callbacks},
        event,
        utils::{string_from_cstr, FromSys},
        *,
    };
    pub(crate) use crossbeam_channel::{Receiver, Sender};
    pub(crate) use discord_game_sdk_sys as sys;
    pub(crate) use std::{
        collections::HashMap,
        ffi::{c_void, CStr, CString},
        mem::size_of,
    };
}

pub use crate::{
    activity::{Action, Activity, ActivityChange, ActivityKind, RequestReply},
    discord::{CreateFlags, Discord},
    entitlement::{Entitlement, EntitlementKind},
    error::{Error, Result},
    file::FileStat,
    lobby::{Lobby, LobbyKind},
    lobby_transaction::{LobbyMemberTransaction, LobbyTransaction},
    oauth2_token::OAuth2Token,
    relationship::{Presence, Relationship, RelationshipKind, Status},
    sku::{Sku, SkuKind},
    user::{PremiumType, User},
};
