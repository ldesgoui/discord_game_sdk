use crate::{sys, to_result::ToResult, utils::MacroHelper, Cast, Comparison, Distance, Result};

/// Lobby Search
///
/// > [Struct in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#data-models-lobbysearchquery-struct)
#[derive(Clone, Debug, Default)]
pub struct SearchQuery {
    pub(crate) filter: Option<(String, String, Comparison, Cast)>,
    pub(crate) sort: Option<(String, String, Cast)>,
    pub(crate) limit: Option<u32>,
    pub(crate) distance: Option<Distance>,
}

impl SearchQuery {
    /// Creates a search object to search available lobbies.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#getsearchquery)
    pub fn new() -> Self {
        Self::default()
    }

    /// Filters lobbies based on metadata comparison.
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `key` and `value` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#lobbysearchfilter)
    pub fn filter(
        &mut self,
        mut key: String,
        comparison: Comparison,
        mut value: String,
        cast: Cast,
    ) -> &mut Self {
        if !key.ends_with('\0') {
            key.push('\0')
        };

        if !value.ends_with('\0') {
            value.push('\0')
        };

        self.filter = Some((key, value, comparison, cast));
        self
    }

    /// Sorts the filtered lobbies based on "near-ness" to a given value
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `key` and `value` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#lobbysearchsort)
    pub fn sort(&mut self, mut key: String, mut value: String, cast: Cast) -> &mut Self {
        if !key.ends_with('\0') {
            key.push('\0')
        };

        if !value.ends_with('\0') {
            value.push('\0')
        };

        self.sort = Some((key, value, cast));
        self
    }

    /// Limits the number of lobbies returned in a search
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#lobbysearchlimit)
    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Filters lobby results to within certain regions relative to the user's location
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/lobbies#lobbysearchdistance)
    pub fn distance(&mut self, distance: Distance) -> &mut Self {
        self.distance = Some(distance);
        self
    }

    pub(crate) unsafe fn process(&self, ptr: *mut sys::IDiscordLobbySearchQuery) -> Result<()> {
        let tx = MacroHelper::new(ptr);

        if let Some((key, value, comparison, cast)) = self.filter.as_ref() {
            ffi!(tx.filter(
                // XXX: *mut should be *const
                key.as_ptr() as *mut u8,
                (*comparison).into(),
                (*cast).into(),
                // XXX: *mut should be *const
                value.as_ptr() as *mut u8,
            ))
            .to_result()?;
        }

        if let Some((key, value, cast)) = self.sort.as_ref() {
            ffi!(tx.sort(
                // XXX: *mut should be *const
                key.as_ptr() as *mut u8,
                (*cast).into(),
                // XXX: *mut should be *const
                value.as_ptr() as *mut u8,
            ))
            .to_result()?;
        }

        if let Some(limit) = self.limit {
            ffi!(tx.limit(limit)).to_result()?;
        }

        if let Some(distance) = self.distance {
            ffi!(tx.distance(distance.into())).to_result()?;
        }

        Ok(())
    }
}
