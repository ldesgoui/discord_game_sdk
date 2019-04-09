#![allow(unused_variables)]

#[macro_use]
mod macros;

pub mod instance;
pub mod interfaces;
pub mod state;

pub mod methods {
    pub mod achievements;
    pub mod activities;
    pub mod applications;
    pub mod core;
    pub mod images;
    pub mod lobbies;
    pub mod networking;
    pub mod overlay;
    pub mod relationships;
    pub mod storage;
    pub mod store;
    pub mod users;
    pub mod voice;
}

pub mod prelude {
    pub use crate::{instance::Instance, interfaces::Interfaces, state::State};
    pub use discord_game_sdk_sys as sys;
    pub use std::ffi::*;
}

pub use methods::core::DiscordCreate;
