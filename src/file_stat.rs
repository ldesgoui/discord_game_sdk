use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileStat {
    pub filename: String,
    pub size: u64,
    pub last_modified: chrono::DateTime<chrono::Utc>,
}

impl FromSys for FileStat {
    type Source = sys::DiscordFileStat;

    fn from_sys(source: &Self::Source) -> Self {
        use chrono::offset::TimeZone;

        Self {
            filename: unsafe { string_from_cstr(&source.filename as *const _) },
            size: source.size,
            last_modified: chrono::Utc.timestamp(source.last_modified as i64, 0),
        }
    }
}
