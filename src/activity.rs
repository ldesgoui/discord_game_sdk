use crate::{sys, ActivityKind};

/// Activity (also known as Rich Presence)
///
/// <https://discordapp.com/developers/docs/game-sdk/activities#data-models-activity-struct>
#[derive(Clone, Copy, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct Activity(pub(crate) sys::DiscordActivity);

impl Activity {
    /// Create a new Activity with empty fields
    pub fn empty() -> Self {
        Self(sys::DiscordActivity::default())
    }

    /// Check if an Activity is completely blank and should be ignored
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

    /// UTC Timestamp
    pub fn start_time(&self) -> i64 {
        self.0.timestamps.start
    }

    /// UTC Timestamp
    pub fn end_time(&self) -> i64 {
        self.0.timestamps.end
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

    pub fn with_start_time(&'_ mut self, value: i64) -> &'_ mut Self {
        self.0.timestamps.start = value;
        self
    }

    pub fn with_end_time(&'_ mut self, value: i64) -> &'_ mut Self {
        self.0.timestamps.end = value;
        self
    }

    set_str!(with_large_image_key, assets.large_image);
    set_str!(with_large_image_tooltip, assets.large_text);
    set_str!(with_small_image_key, assets.small_image);
    set_str!(with_small_image_tooltip, assets.small_text);

    set_str!(with_party_id, party.id);

    pub fn with_party_amount(&'_ mut self, value: i32) -> &'_ mut Self {
        self.0.party.size.current_size = value;
        self
    }

    pub fn with_party_capacity(&'_ mut self, value: i32) -> &'_ mut Self {
        self.0.party.size.max_size = value;
        self
    }

    pub fn with_instance(&'_ mut self, value: bool) -> &'_ mut Self {
        self.0.instance = value;
        self
    }

    set_str!(with_match_secret, secrets.match_);
    set_str!(with_join_secret, secrets.join);
    set_str!(with_spectate_secret, secrets.spectate);
}

impl std::fmt::Debug for Activity {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Activity")
            .field("kind", &self.kind())
            .field("application_id", &self.application_id())
            .field("name", &self.name())
            .field("state", &self.state())
            .field("details", &self.details())
            .field("start_time", &self.start_time())
            .field("end_time", &self.end_time())
            .field("large_image_key", &self.large_image_key())
            .field("large_image_tooltip", &self.large_image_tooltip())
            .field("small_image_key", &self.small_image_key())
            .field("small_image_tooltip", &self.small_image_tooltip())
            .field("party_id", &self.party_id())
            .field("party_amount", &self.party_amount())
            .field("party_capacity", &self.party_capacity())
            .field("instance", &self.instance())
            .field("match_secret", &self.match_secret())
            .field("join_secret", &self.join_secret())
            .field("spectate_secret", &self.spectate_secret())
            .finish()
    }
}
