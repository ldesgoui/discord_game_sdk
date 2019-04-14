use crate::sys;
use chrono::{offset::TimeZone, DateTime, Utc};

#[derive(Clone, Copy, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct FileStat(pub(crate) sys::DiscordFileStat);

impl FileStat {
    get_str!(filename, filename);

    pub fn size(&self) -> u64 {
        self.0.size
    }

    pub fn last_modified(&self) -> DateTime<Utc> {
        Utc.timestamp(self.0.last_modified as i64, 0)
    }
}

impl std::fmt::Debug for FileStat {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("FileStat")
            .field("filename", &self.filename())
            .field("size", &self.size())
            .field("last_modified", &self.last_modified())
            .finish()
    }
}
