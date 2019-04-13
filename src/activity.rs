use crate::{sys, ActivityKind};
use chrono::{offset::TimeZone, DateTime, Utc};

#[derive(Clone, Copy, Debug, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct Activity(pub(crate) sys::DiscordActivity);

impl Activity {
    pub fn kind(&self) -> ActivityKind {
        self.0.type_.into()
    }

    pub fn application_id(&self) -> i64 {
        self.0.application_id
    }

    str_field!(name, name);
    str_field!(state, state);
    str_field!(details, details);

    pub fn start_time(&self) -> DateTime<Utc> {
        Utc.timestamp(self.0.timestamps.start, 0)
    }

    pub fn end_time(&self) -> DateTime<Utc> {
        Utc.timestamp(self.0.timestamps.end, 0)
    }

    str_field!(large_image_key, assets.large_image);
    str_field!(large_image_tooltip, assets.large_text);
    str_field!(small_image_key, assets.small_image);
    str_field!(small_image_tooltip, assets.small_text);

    str_field!(party_id, party.id);

    pub fn party_amount(&self) -> i32 {
        self.0.party.size.current_size
    }

    pub fn party_capacity(&self) -> i32 {
        self.0.party.size.max_size
    }

    pub fn instance(&self) -> bool {
        self.0.instance
    }

    str_field!(match_secret, secrets.match_);
    str_field!(join_secret, secrets.join);
    str_field!(spectate_secret, secrets.spectate);
}
