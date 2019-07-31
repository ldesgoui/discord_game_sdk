use crate::{
    macro_helper::MacroHelper, sys, to_result::ToResult, Cast, Comparison, Distance, Result,
};
use std::ffi::CStr;

/// Lobby Search
///
/// <https://discordapp.com/developers/docs/game-sdk/lobbies#search>
#[derive(Clone, Debug, Default)]
pub struct SearchQuery<'a> {
    pub(crate) filter: Option<(&'a CStr, &'a CStr, Comparison, Cast)>,
    pub(crate) sort: Option<(&'a CStr, &'a CStr, Cast)>,
    pub(crate) limit: Option<u32>,
    pub(crate) distance: Option<Distance>,
}

impl<'a> SearchQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Filters lobbies based on metadata comparison
    ///
    /// `key` and `value` must also be valid UTF-8
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/lobbies#lobbysearchfilter>
    pub fn filter(
        &mut self,
        key: &'a CStr,
        comparison: Comparison,
        value: &'a CStr,
        cast: Cast,
    ) -> &mut Self {
        self.filter = Some((key, value, comparison, cast));
        self
    }

    /// Sorts the filtered lobbies based on "near-ness" to a given value
    ///
    /// `key` and `value` must also be valid UTF-8
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/lobbies#lobbysearchsort>
    pub fn sort(&mut self, key: &'a CStr, value: &'a CStr, cast: Cast) -> &mut Self {
        self.sort = Some((key, value, cast));
        self
    }

    /// Limits the number of lobbies returned in a search
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/lobbies#lobbysearchlimit>
    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Filters lobby results to within certain regions relative to the user's location
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/lobbies#lobbysearchdistance>
    pub fn distance(&mut self, distance: Distance) -> &mut Self {
        self.distance = Some(distance);
        self
    }

    pub(crate) unsafe fn process(&self, ptr: *mut sys::IDiscordLobbySearchQuery) -> Result<()> {
        let tx = MacroHelper { core: ptr };

        if let Some((key, value, comparison, cast)) = self.filter {
            ffi!(tx.filter(
                key.as_ptr() as *mut _,
                comparison.into(),
                cast.into(),
                value.as_ptr() as *mut _,
            ))
            .to_result()?;
        }

        if let Some((key, value, cast)) = self.sort {
            ffi!(tx.sort(
                key.as_ptr() as *mut _,
                cast.into(),
                value.as_ptr() as *mut _,
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
