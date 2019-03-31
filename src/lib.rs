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

#![warn(
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    // missing_docs
)]
#![allow(
    clippy::implicit_return,
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    clippy::module_name_repetitions,
    clippy::multiple_inherent_impl,
    unused_variables
)]

//

#[macro_use]
mod macros;

mod core;

mod application;

pub mod error;
mod events;

//

pub use crate::core::{CreateFlags, Discord};
pub use crate::error::{Error, Result};

//

#[cfg(test)]
mod smoke {
    use super::*;

    #[test]
    fn compiles() {
        let _ = pretty_env_logger::try_init_custom_env("TEST_LOG");

        let mut gsdk = Discord::new(0).unwrap();
        log::info!("{:?}", gsdk);
        gsdk.run_callbacks().unwrap();
        gsdk.validate_or_exit(&mut |r| match r {
            Ok(()) => log::info!("Validated"),
            Err(err) => log::error!("Exiting! {}", err),
        });
        log::info!("{:?}", gsdk.get_current_locale());
        log::info!("{:?}", gsdk.get_current_branch());
    }

}
