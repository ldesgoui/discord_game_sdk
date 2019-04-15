//! Drop-in mock replacement for the [Discord Game SDK]
//!
//! ## Status
//!
//! This is currently a work-in-progress, most functions will accept calls
//! without providing meaningful or valid results.
//! The plan is to provide an interface for the developer to build scenarios,
//! allowing for a mish-mash of unit testing and integration testing, with the
//! major benefits that all happens in memory rather than hitting the network.
//!
//! [Discord Game SDK]: https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide

#![allow(unused_variables)]

#[macro_use]
mod macros;

pub(crate) mod instance;
pub(crate) mod interfaces;
pub(crate) mod state;

pub(crate) mod methods {
    pub(crate) mod achievements;
    pub(crate) mod activities;
    pub(crate) mod applications;
    pub(crate) mod core;
    pub(crate) mod images;
    pub(crate) mod lobbies;
    pub(crate) mod networking;
    pub(crate) mod overlay;
    pub(crate) mod relationships;
    pub(crate) mod storage;
    pub(crate) mod store;
    pub(crate) mod users;
    pub(crate) mod voice;
}

pub(crate) mod prelude {
    pub(crate) use crate::{instance::Instance, interfaces::Interfaces, state::State};
    pub(crate) use discord_game_sdk_sys as sys;
    pub(crate) use std::ffi::*;
}
