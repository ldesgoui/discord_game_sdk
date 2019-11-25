use crate::{
    callbacks::{ResultBytesCallback, ResultCallback},
    iter, sys,
    to_result::ToResult,
    utils::charbuf_to_str,
    Discord, FileStat, Result,
};
use std::mem::size_of;

/// # Storage
///
/// <https://discordapp.com/developers/docs/game-sdk/storage>
impl<'a> Discord<'a> {
    /// Reads data synchronously from the game's allocated save file into a buffer.
    /// The file is mapped by key-value pair, and this function will read data that exists
    /// for the given key name.
    ///
    /// `filename` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/storage#read>
    pub fn read_file(&self, mut filename: String, mut buffer: impl AsMut<[u8]>) -> Result<u32> {
        filename.push('\0');

        let mut read = 0;

        let buffer = buffer.as_mut();

        unsafe {
            ffi!(self.get_storage_manager().read(
                filename.as_ptr() as *const _,
                buffer.as_mut_ptr(),
                std::cmp::min(buffer.len(), u32::max_value() as usize) as u32,
                &mut read as *mut _
            ))
        }
        .to_result()?;

        Ok(read)
    }

    /// Reads data asynchronously from the game's allocated save file into a buffer.
    ///
    /// `filename` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/storage#readasync>
    pub fn read_file_async(
        &mut self,
        mut filename: String,
        callback: impl FnMut(&mut Discord, Result<Vec<u8>>) + 'a,
    ) {
        filename.push('\0');

        unsafe {
            ffi!(self
                .get_storage_manager()
                .read_async(filename.as_ptr() as *const _)
                .and_then(ResultBytesCallback::new(callback)))
        }
    }

    /// Reads data asynchronously from the game's allocated save file into a buffer,
    /// starting at a given offset and up to a given length.
    ///
    /// `filename` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/storage#readasyncpartial>
    pub fn read_file_async_partial(
        &mut self,
        mut filename: String,
        offset: u64,
        length: u64,
        callback: impl FnMut(&mut Discord, Result<Vec<u8>>) + 'a,
    ) {
        filename.push('\0');

        unsafe {
            ffi!(self
                .get_storage_manager()
                .read_async_partial(filename.as_ptr() as *const _, offset, length)
                .and_then(ResultBytesCallback::new(callback)))
        }
    }

    /// Writes data synchronously to disk, under the given key name.
    ///
    /// `filename` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/storage#write>
    pub fn write_file(&self, mut filename: String, buffer: impl AsRef<[u8]>) -> Result<()> {
        filename.push('\0');

        let buffer = buffer.as_ref();

        unsafe {
            ffi!(self.get_storage_manager().write(
                filename.as_ptr() as *const _,
                buffer.as_ptr() as *mut _,
                buffer.len() as u32,
            ))
        }
        .to_result()
    }

    /// Writes data asynchronously to disk under the given key.
    ///
    /// `filename` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/storage#writeasync>
    pub fn write_file_async(
        &mut self,
        mut filename: String,
        buffer: impl AsRef<[u8]>,
        callback: impl FnMut(&mut Discord, Result<()>) + 'a,
    ) {
        filename.push('\0');

        let buffer = buffer.as_ref();

        unsafe {
            ffi!(self
                .get_storage_manager()
                .write_async(
                    filename.as_ptr() as *const _,
                    buffer.as_ptr() as *mut _,
                    buffer.len() as u32
                )
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// Deletes written data for the given key.
    ///
    /// `filename` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/storage#delete>
    pub fn delete_file(&self, mut filename: String) -> Result<()> {
        filename.push('\0');

        unsafe {
            ffi!(self
                .get_storage_manager()
                .delete_(filename.as_ptr() as *const _))
        }
        .to_result()
    }

    /// Checks if data exists for a given key.
    ///
    /// `filename` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/storage#exists>
    pub fn file_exists(&self, mut filename: String) -> Result<bool> {
        filename.push('\0');

        let mut exists = false;

        unsafe {
            ffi!(self
                .get_storage_manager()
                .exists(filename.as_ptr() as *const _, &mut exists))
        }
        .to_result()?;

        Ok(exists)
    }

    /// Returns file info for the given key.
    ///
    /// `filename` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/storage#stat>
    pub fn file_stat(&self, mut filename: String) -> Result<FileStat> {
        filename.push('\0');

        let mut stat = FileStat(sys::DiscordFileStat::default());

        unsafe {
            ffi!(self
                .get_storage_manager()
                .stat(filename.as_ptr() as *const _, &mut stat.0))
        }
        .to_result()?;

        Ok(stat)
    }

    /// Returns infos to all existing files.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/storage#statat>  
    pub fn file_stat_count(&self) -> i32 {
        let mut count = 0;

        unsafe { ffi!(self.get_storage_manager().count(&mut count)) }

        count
    }

    /// <https://discordapp.com/developers/docs/game-sdk/storage#count>
    pub fn file_stat_at(&self, index: i32) -> Result<FileStat> {
        let mut stat = FileStat(sys::DiscordFileStat::default());

        unsafe {
            ffi!(self
                .get_storage_manager()
                .stat_at(index as i32, &mut stat.0))
        }
        .to_result()?;

        Ok(stat)
    }

    pub fn iter_file_stats<'b>(&'b self) -> iter::GenericIter<'a, 'b, Result<FileStat>> {
        let count = self.file_stat_count();

        iter::GenericIter::new(self, Box::new(|d, i| d.file_stat_at(i)), count)
    }

    /// Returns the path to the folder where files are stored.
    /// It is specific to the application ID, the current branch, and the current user.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/storage#getpath>
    pub fn folder_path(&self) -> Result<String> {
        let mut path: sys::DiscordPath = [0; size_of::<sys::DiscordPath>()];

        unsafe { ffi!(self.get_storage_manager().get_path(&mut path)) }.to_result()?;

        Ok(charbuf_to_str(&path).to_string())
    }
}
