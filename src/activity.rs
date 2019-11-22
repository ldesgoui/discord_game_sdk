use crate::{
    sys,
    utils::{charbuf_len, charbuf_to_str, write_charbuf},
    ActivityKind,
};

/// Activity (also known as Rich Presence)
///
/// <https://discordapp.com/developers/docs/game-sdk/activities#data-models-activity-struct>
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Activity {
    pub(crate) sys: sys::DiscordActivity,
    name_len: usize,
    state_len: usize,
    details_len: usize,
    large_image_key_len: usize,
    large_image_tooltip_len: usize,
    small_image_key_len: usize,
    small_image_tooltip_len: usize,
    party_id_len: usize,
    match_secret_len: usize,
    join_secret_len: usize,
    spectate_secret_len: usize,
}

impl Activity {
    /// Create a new Activity with empty fields
    pub fn empty() -> Self {
        Self::from(sys::DiscordActivity::default())
    }

    /// Check if an Activity is completely blank and should be ignored
    pub fn is_empty(&self) -> bool {
        self == &Self::empty()
    }

    pub fn kind(&self) -> ActivityKind {
        self.sys.type_.into()
    }

    pub fn application_id(&self) -> i64 {
        self.sys.application_id
    }

    pub fn name(&self) -> &str {
        charbuf_to_str(&self.sys.name[..self.name_len])
    }

    pub fn state(&self) -> &str {
        charbuf_to_str(&self.sys.state[..self.state_len])
    }

    pub fn details(&self) -> &str {
        charbuf_to_str(&self.sys.details[..self.details_len])
    }

    /// UTC Timestamp
    pub fn start_time(&self) -> i64 {
        self.sys.timestamps.start
    }

    /// UTC Timestamp
    pub fn end_time(&self) -> i64 {
        self.sys.timestamps.end
    }

    pub fn large_image_key(&self) -> &str {
        charbuf_to_str(&self.sys.assets.large_image[..self.large_image_key_len])
    }

    pub fn large_image_tooltip(&self) -> &str {
        charbuf_to_str(&self.sys.assets.large_text[..self.large_image_tooltip_len])
    }

    pub fn small_image_key(&self) -> &str {
        charbuf_to_str(&self.sys.assets.small_image[..self.small_image_key_len])
    }

    pub fn small_image_tooltip(&self) -> &str {
        charbuf_to_str(&self.sys.assets.small_text[..self.small_image_tooltip_len])
    }

    pub fn party_id(&self) -> &str {
        charbuf_to_str(&self.sys.party.id[..self.party_id_len])
    }

    pub fn party_amount(&self) -> i32 {
        self.sys.party.size.current_size
    }

    pub fn party_capacity(&self) -> i32 {
        self.sys.party.size.max_size
    }

    pub fn instance(&self) -> bool {
        self.sys.instance
    }

    pub fn match_secret(&self) -> &str {
        charbuf_to_str(&self.sys.secrets.match_[..self.match_secret_len])
    }

    pub fn join_secret(&self) -> &str {
        charbuf_to_str(&self.sys.secrets.join[..self.join_secret_len])
    }

    pub fn spectate_secret(&self) -> &str {
        charbuf_to_str(&self.sys.secrets.spectate[..self.spectate_secret_len])
    }

    /// `value` *MUST NOT* contain nul bytes
    pub fn with_state(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.sys.state, value);
        self.state_len = value.len();
        self
    }

    /// `value` *MUST NOT* contain nul bytes
    pub fn with_details(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.sys.details, value);
        self.details_len = value.len();
        self
    }

    /// `value` is an UTC timestamp
    pub fn with_start_time(&'_ mut self, value: i64) -> &'_ mut Self {
        self.sys.timestamps.start = value;
        self
    }

    /// `value` is an UTC timestamp
    pub fn with_end_time(&'_ mut self, value: i64) -> &'_ mut Self {
        self.sys.timestamps.end = value;
        self
    }

    /// `value` *MUST NOT* contain nul bytes
    pub fn with_large_image_key(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.sys.assets.large_image, value);
        self.large_image_key_len = value.len();
        self
    }

    /// `value` *MUST NOT* contain nul bytes
    pub fn with_large_image_tooltip(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.sys.assets.large_text, value);
        self.large_image_tooltip_len = value.len();
        self
    }

    /// `value` *MUST NOT* contain nul bytes
    pub fn with_small_image_key(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.sys.assets.small_image, value);
        self.small_image_key_len = value.len();
        self
    }

    /// `value` *MUST NOT* contain nul bytes
    pub fn with_small_image_tooltip(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.sys.assets.small_text, value);
        self.small_image_tooltip_len = value.len();
        self
    }

    /// `value` *MUST NOT* contain nul bytes
    pub fn with_party_id(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.sys.party.id, value);
        self.party_id_len = value.len();
        self
    }

    pub fn with_party_amount(&'_ mut self, value: i32) -> &'_ mut Self {
        self.sys.party.size.current_size = value;
        self
    }

    pub fn with_party_capacity(&'_ mut self, value: i32) -> &'_ mut Self {
        self.sys.party.size.max_size = value;
        self
    }

    pub fn with_instance(&'_ mut self, value: bool) -> &'_ mut Self {
        self.sys.instance = value;
        self
    }

    /// `value` *MUST NOT* contain nul bytes
    pub fn with_match_secret(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.sys.secrets.match_, value);
        self.match_secret_len = value.len();
        self
    }

    /// `value` *MUST NOT* contain nul bytes
    pub fn with_join_secret(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.sys.secrets.join, value);
        self.join_secret_len = value.len();
        self
    }

    /// `value` *MUST NOT* contain nul bytes
    pub fn with_spectate_secret(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.sys.secrets.spectate, value);
        self.spectate_secret_len = value.len();
        self
    }
}

impl From<sys::DiscordActivity> for Activity {
    fn from(sys: sys::DiscordActivity) -> Self {
        Self {
            sys,
            name_len: charbuf_len(&sys.name),
            state_len: charbuf_len(&sys.state),
            details_len: charbuf_len(&sys.details),
            large_image_key_len: charbuf_len(&sys.assets.large_image),
            large_image_tooltip_len: charbuf_len(&sys.assets.large_text),
            small_image_key_len: charbuf_len(&sys.assets.small_image),
            small_image_tooltip_len: charbuf_len(&sys.assets.small_text),
            party_id_len: charbuf_len(&sys.party.id),
            match_secret_len: charbuf_len(&sys.secrets.match_),
            join_secret_len: charbuf_len(&sys.secrets.join),
            spectate_secret_len: charbuf_len(&sys.secrets.spectate),
        }
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
