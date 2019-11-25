use crate::{sys, utils::charbuf_to_str};

/// File Metadata
///
/// <https://discordapp.com/developers/docs/game-sdk/storage#data-models-filestat-struct>
#[derive(Clone, Copy, Eq, PartialEq, derive_more::From, derive_more::Into)]
#[repr(transparent)]
pub struct FileStat(pub(crate) sys::DiscordFileStat);

impl FileStat {
    /// The name of the file
    pub fn filename(&self) -> &str {
        charbuf_to_str(&self.0.filename)
    }

    /// The total size in bytes
    pub fn size(&self) -> u64 {
        self.0.size
    }

    /// When the file was last modified, in UNIX Time
    pub fn last_modified(&self) -> i64 {
        self.0.last_modified as i64
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
