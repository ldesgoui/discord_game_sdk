use crate::{
    macro_helper::MacroHelper, sys, to_result::ToResult, Cast, Comparison, Distance, Result,
};
use std::ffi::CStr;

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

    pub fn filter(
        &'a mut self,
        key: &'a CStr,
        comparison: Comparison,
        value: &'a CStr,
        cast: Cast,
    ) -> &'a mut Self {
        self.filter = Some((key, value, comparison, cast));
        self
    }

    pub fn sort(&'a mut self, key: &'a CStr, value: &'a CStr, cast: Cast) -> &'a mut Self {
        self.sort = Some((key, value, cast));
        self
    }

    pub fn limit(&'a mut self, limit: u32) -> &'a mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn distance(&'a mut self, distance: Distance) -> &'a mut Self {
        self.distance = Some(distance);
        self
    }

    pub(crate) unsafe fn process(self, ptr: *mut sys::IDiscordLobbySearchQuery) -> Result<()> {
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
