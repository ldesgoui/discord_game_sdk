use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileStat {
    pub filename: String,
    pub size: u64,
    pub last_modified: chrono::NaiveDateTime,
}

impl FileStat {
    pub(crate) fn from_sys(source: sys::DiscordFileStat) -> Result<Self> {
        Ok(Self {
            filename: from_cstr(&source.filename as *const _)?.to_string(),
            size: source.size,
            last_modified: chrono::NaiveDateTime::from_timestamp(source.last_modified as i64, 0),
        })
    }
}
