use crate::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct SearchQuery<'a> {
    pub(crate) filter: Option<(&'a str, &'a str, Comparison, Cast)>,
    pub(crate) sort: Option<(&'a str, &'a str, Cast)>,
    pub(crate) limit: Option<u32>,
    pub(crate) distance: Option<Distance>,
}

impl<'a> SearchQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn filter(
        &'a mut self,
        key: &'a str,
        value: &'a str,
        comparison: Comparison,
        cast: Cast,
    ) -> &'a mut Self {
        self.filter = Some((key, value, comparison, cast));
        self
    }

    pub fn sort(&'a mut self, key: &'a str, value: &'a str, cast: Cast) -> &'a mut Self {
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
            let key = CString::new(key).unwrap();
            let value = CString::new(value).unwrap();

            ffi!(tx.filter(
                key.as_ptr() as *mut _,
                comparison.to_sys(),
                cast.to_sys(),
                value.as_ptr() as *mut _,
            ))
            .to_result()?;
        }

        if let Some((key, value, cast)) = self.sort {
            ffi!(tx.sort(
                key.as_ptr() as *mut _,
                cast.to_sys(),
                value.as_ptr() as *mut _,
            ))
            .to_result()?;
        }

        if let Some(limit) = self.limit {
            ffi!(tx.limit(limit)).to_result()?;
        }

        if let Some(distance) = self.distance {
            ffi!(tx.distance(distance.to_sys())).to_result()?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cast {
    Number,
    String,
}

impl Cast {
    fn to_sys(self) -> sys::EDiscordLobbySearchCast {
        match self {
            Cast::String => sys::DiscordLobbySearchCast_String,
            Cast::Number => sys::DiscordLobbySearchCast_Number,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    NotEqual,
}

impl Comparison {
    fn to_sys(self) -> sys::EDiscordLobbySearchComparison {
        match self {
            Comparison::Equal => sys::DiscordLobbySearchComparison_Equal,
            Comparison::GreaterThan => sys::DiscordLobbySearchComparison_GreaterThan,
            Comparison::GreaterThanOrEqual => sys::DiscordLobbySearchComparison_GreaterThanOrEqual,
            Comparison::LessThan => sys::DiscordLobbySearchComparison_LessThan,
            Comparison::LessThanOrEqual => sys::DiscordLobbySearchComparison_LessThanOrEqual,
            Comparison::NotEqual => sys::DiscordLobbySearchComparison_NotEqual,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Distance {
    Default,
    Extended,
    Global,
    Local,
}

impl Distance {
    fn to_sys(self) -> sys::EDiscordLobbySearchDistance {
        match self {
            Distance::Default => sys::DiscordLobbySearchDistance_Default,
            Distance::Extended => sys::DiscordLobbySearchDistance_Extended,
            Distance::Global => sys::DiscordLobbySearchDistance_Global,
            Distance::Local => sys::DiscordLobbySearchDistance_Local,
        }
    }
}
