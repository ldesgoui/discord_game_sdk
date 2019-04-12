use crate::prelude::*;

pub struct SearchQuery<'a> {
    pub(crate) core: &'a mut sys::IDiscordLobbySearchQuery,
}

impl<'a> SearchQuery<'a> {
    //pub fn filter(
    //    &mut self,
    //    key: impl AsRef<str>,
    //    value: impl AsRef<str>,
    //    comparison: Comparison,
    //    cast: Cast,
    //) -> Result<()> {
    //    let key = CString::new(key.as_ref()).unwrap();
    //    let value = CString::new(value.as_ref()).unwrap();

    //    unsafe {
    //        ffi!(self.filter(
    //            key.as_ptr() as *mut _,
    //            comparison.to_sys(),
    //            cast.to_sys(),
    //            value.as_ptr() as *mut _
    //        ))
    //    }
    //    .to_result()
    //}

    //pub fn sort(&mut self, key: impl AsRef<str>, value: impl AsRef<str>, cast: Cast) -> Result<()> {
    //    let key = CString::new(key.as_ref()).unwrap();
    //    let value = CString::new(value.as_ref()).unwrap();

    //    unsafe {
    //        ffi!(self.sort(
    //            key.as_ptr() as *mut _,
    //            cast.to_sys(),
    //            value.as_ptr() as *mut _
    //        ))
    //    }
    //    .to_result()
    //}

    //pub fn limit(&mut self, limit: u32) -> Result<()> {
    //    unsafe { ffi!(self.limit(limit)) }.to_result()
    //}

    //pub fn distance(&mut self, distance: Distance) -> Result<()> {
    //    unsafe { ffi!(self.distance(distance.to_sys())) }.to_result()
    //}
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
