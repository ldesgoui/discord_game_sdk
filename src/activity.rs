use crate::{sys, ActivityKind};
use chrono::{offset::TimeZone, DateTime, Utc};

#[derive(Clone, Copy, Debug, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct Activity(pub(crate) sys::DiscordActivity);

impl Activity {
    pub fn empty() -> Self {
        Self(sys::DiscordActivity::default())
    }

    pub fn is_empty(&self) -> bool {
        self == &Self::empty()
    }

    pub fn kind(&self) -> ActivityKind {
        self.0.type_.into()
    }

    pub fn application_id(&self) -> i64 {
        self.0.application_id
    }

    get_str!(name, name);
    get_str!(state, state);
    get_str!(details, details);

    pub fn start_time(&self) -> DateTime<Utc> {
        Utc.timestamp(self.0.timestamps.start, 0)
    }

    pub fn end_time(&self) -> DateTime<Utc> {
        Utc.timestamp(self.0.timestamps.end, 0)
    }

    get_str!(large_image_key, assets.large_image);
    get_str!(large_image_tooltip, assets.large_text);
    get_str!(small_image_key, assets.small_image);
    get_str!(small_image_tooltip, assets.small_text);

    get_str!(party_id, party.id);

    pub fn party_amount(&self) -> i32 {
        self.0.party.size.current_size
    }

    pub fn party_capacity(&self) -> i32 {
        self.0.party.size.max_size
    }

    pub fn instance(&self) -> bool {
        self.0.instance
    }

    get_str!(match_secret, secrets.match_);
    get_str!(join_secret, secrets.join);
    get_str!(spectate_secret, secrets.spectate);

    set_str!(with_state, state);
    set_str!(with_details, details);

    pub fn with_start_time<'a>(&'a mut self, value: DateTime<Utc>) -> &'a mut Self {
        self.0.timestamps.start = value.timestamp();
        self
    }

    pub fn with_end_time<'a>(&'a mut self, value: DateTime<Utc>) -> &'a mut Self {
        self.0.timestamps.end = value.timestamp();
        self
    }

    set_str!(with_large_image_key, assets.large_image);
    set_str!(with_large_image_tooltip, assets.large_text);
    set_str!(with_small_image_key, assets.small_image);
    set_str!(with_small_image_tooltip, assets.small_text);

    set_str!(with_party_id, party.id);

    pub fn with_party_amount<'a>(&'a mut self, value: i32) -> &'a mut Self {
        self.0.party.size.current_size = value;
        self
    }

    pub fn with_party_capacity<'a>(&'a mut self, value: i32) -> &'a mut Self {
        self.0.party.size.max_size = value;
        self
    }

    pub fn with_instance<'a>(&'a mut self, value: bool) -> &'a mut Self {
        self.0.instance = value;
        self
    }

    set_str!(with_match_secret, secrets.match_);
    set_str!(with_join_secret, secrets.join);
    set_str!(with_spectate_secret, secrets.spectate);
}
