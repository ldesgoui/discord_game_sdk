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

// We'd like to have this come second so that the methods show up first in the documentation
mod core;

mod activities;
mod application;
pub mod error;
pub mod events;
mod relationships;
mod utils;

pub use crate::activities::{Action, ActivityChange, RequestReply};
pub use crate::application::OAuth2Token;
pub use crate::core::{CreateFlags, Discord};
pub use crate::error::{Error, Result};
