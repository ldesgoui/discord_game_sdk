use crate::sys;
use chrono::{offset::TimeZone, DateTime, Utc};

#[derive(Clone, Copy, Debug, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct FileStat(pub(crate) sys::DiscordFileStat);

impl FileStat {
    str_field!(filename, filename);

    pub fn size(&self) -> u64 {
        self.0.size
    }

    pub fn last_modified(&self) -> DateTime<Utc> {
        Utc.timestamp(self.0.last_modified as i64, 0)
    }
}
