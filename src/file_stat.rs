use crate::{
    sys,
    utils::{charbuf_len, charbuf_to_str},
};

/// File Metadata
///
/// <https://discordapp.com/developers/docs/game-sdk/storage#data-models-filestat-struct>
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct FileStat {
    pub(crate) sys: sys::DiscordFileStat,
    filename_len: usize,
}

impl FileStat {
    pub fn filename(&self) -> &str {
        charbuf_to_str(&self.sys.filename[..self.filename_len])
    }

    pub fn size(&self) -> u64 {
        self.sys.size
    }

    /// UTC Timestamp
    pub fn last_modified(&self) -> i64 {
        self.sys.last_modified as i64
    }
}

impl From<sys::DiscordFileStat> for FileStat {
    fn from(sys: sys::DiscordFileStat) -> Self {
        Self {
            sys,
            filename_len: charbuf_len(&sys.filename),
        }
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
