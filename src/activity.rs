use crate::{
    sys,
    utils::{charbuf_to_str, write_charbuf},
    ActivityKind,
};

/// Activity (also known as Rich Presence)
///
/// <https://discordapp.com/developers/docs/game-sdk/activities#data-models-activity-struct>
#[derive(Clone, Copy, Eq, PartialEq, derive_more::From, derive_more::Into)]
#[repr(transparent)]
pub struct Activity(pub(crate) sys::DiscordActivity);

impl Activity {
    /// Create a new Activity with empty fields
    pub fn empty() -> Self {
        Self::from(sys::DiscordActivity::default())
    }

    /// Check if an Activity is completely blank
    pub fn is_empty(&self) -> bool {
        self == &Self::empty()
    }

    /// Type of Activty
    pub fn kind(&self) -> ActivityKind {
        self.0.type_.into()
    }

    /// The unique ID of the application
    pub fn application_id(&self) -> i64 {
        self.0.application_id
    }

    /// The name of the application
    pub fn name(&self) -> &str {
        charbuf_to_str(&self.0.name)
    }

    /// The player's current party status
    pub fn state(&self) -> &str {
        charbuf_to_str(&self.0.state)
    }

    /// What the player is currently doing
    pub fn details(&self) -> &str {
        charbuf_to_str(&self.0.details)
    }

    /// When the current activity has started, in UNIX Time
    pub fn start_time(&self) -> i64 {
        self.0.timestamps.start
    }

    /// When the current activity will end, in UNIX Time
    pub fn end_time(&self) -> i64 {
        self.0.timestamps.end
    }

    /// The key of an asset to display
    pub fn large_image_key(&self) -> &str {
        charbuf_to_str(&self.0.assets.large_image)
    }

    /// The tooltip displayed when hovering over the large image
    pub fn large_image_tooltip(&self) -> &str {
        charbuf_to_str(&self.0.assets.large_text)
    }

    /// The key of an asset to display
    pub fn small_image_key(&self) -> &str {
        charbuf_to_str(&self.0.assets.small_image)
    }

    /// The tooltip displayed when hovering over the small image
    pub fn small_image_tooltip(&self) -> &str {
        charbuf_to_str(&self.0.assets.small_text)
    }

    /// The unique identifier for the party
    pub fn party_id(&self) -> &str {
        charbuf_to_str(&self.0.party.id)
    }

    /// The number of players currently in the party
    pub fn party_amount(&self) -> i32 {
        self.0.party.size.current_size
    }

    /// The maximum capacity of the party
    pub fn party_capacity(&self) -> i32 {
        self.0.party.size.max_size
    }

    /// Whether this activity is an instanced context, like a match
    pub fn instance(&self) -> bool {
        self.0.instance
    }

    /// The unique hash for the given match context
    pub fn match_secret(&self) -> &str {
        charbuf_to_str(&self.0.secrets.match_)
    }

    /// The unique hash for chat invites and Ask to Join
    pub fn join_secret(&self) -> &str {
        charbuf_to_str(&self.0.secrets.join)
    }

    /// The unique hash for Spectate button
    pub fn spectate_secret(&self) -> &str {
        charbuf_to_str(&self.0.secrets.spectate)
    }

    /// The player's current party status
    ///
    /// `value` *MUST NOT* contain nul bytes
    pub fn with_state(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.0.state, value);
        self
    }

    /// What the player is currently doing
    ///
    /// `value` *MUST NOT* contain nul bytes
    pub fn with_details(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.0.details, value);
        self
    }

    /// When the current activity has started, in UNIX time
    pub fn with_start_time(&'_ mut self, value: i64) -> &'_ mut Self {
        self.0.timestamps.start = value;
        self
    }

    /// When the current activity will end, in UNIX time
    pub fn with_end_time(&'_ mut self, value: i64) -> &'_ mut Self {
        self.0.timestamps.end = value;
        self
    }

    /// The key of an asset to display
    ///
    /// `value` *MUST NOT* contain nul bytes
    pub fn with_large_image_key(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.0.assets.large_image, value);
        self
    }

    /// The tooltip displayed when hovering over the large image
    ///
    /// `value` *MUST NOT* contain nul bytes
    pub fn with_large_image_tooltip(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.0.assets.large_text, value);
        self
    }

    /// The key of an asset to display
    ///
    /// `value` *MUST NOT* contain nul bytes
    pub fn with_small_image_key(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.0.assets.small_image, value);
        self
    }

    /// The tooltip displayed when hovering over the small image
    ///
    /// `value` *MUST NOT* contain nul bytes
    pub fn with_small_image_tooltip(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.0.assets.small_text, value);
        self
    }

    /// The unique identifier for the party
    ///
    /// `value` *MUST NOT* contain nul bytes
    pub fn with_party_id(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.0.party.id, value);
        self
    }

    /// The number of players currently in the party
    pub fn with_party_amount(&'_ mut self, value: i32) -> &'_ mut Self {
        self.0.party.size.current_size = value;
        self
    }

    /// The maximum capacity of the party
    pub fn with_party_capacity(&'_ mut self, value: i32) -> &'_ mut Self {
        self.0.party.size.max_size = value;
        self
    }

    /// Whether this activity is an instanced context, like a match
    pub fn with_instance(&'_ mut self, value: bool) -> &'_ mut Self {
        self.0.instance = value;
        self
    }

    /// The unique hash for the given match context
    ///
    /// `value` *MUST NOT* contain nul bytes
    pub fn with_match_secret(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.0.secrets.match_, value);
        self
    }

    /// The unique hash for chat invites and Ask to Join
    ///
    /// `value` *MUST NOT* contain nul bytes
    pub fn with_join_secret(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.0.secrets.join, value);
        self
    }

    /// The unique hash for Spectate button
    ///
    /// `value` *MUST NOT* contain nul bytes
    pub fn with_spectate_secret(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.0.secrets.spectate, value);
        self
    }
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
