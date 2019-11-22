use crate::{
    callbacks::{ResultBytesCallback, ResultCallback},
    sys,
    to_result::ToResult,
    utils::{charbuf_len, charbuf_to_str},
    Discord, FileStat, Result,
};
use std::mem::size_of;

/// # Storage
///
/// <https://discordapp.com/developers/docs/game-sdk/storage>
impl<'a> Discord<'a> {
    /// `filename` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/storage#read>
    pub fn read_file(&mut self, mut filename: String, mut buffer: impl AsMut<[u8]>) -> Result<u32> {
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

    /// `filename` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/storage#write>
    pub fn write_file(&mut self, mut filename: String, buffer: impl AsRef<[u8]>) -> Result<()> {
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

    /// `filename` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/storage#delete>
    pub fn delete_file(&mut self, mut filename: String) -> Result<()> {
        filename.push('\0');

        unsafe {
            ffi!(self
                .get_storage_manager()
                .delete_(filename.as_ptr() as *const _))
        }
        .to_result()
    }

    /// `filename` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/storage#exists>
    pub fn file_exists(&mut self, mut filename: String) -> Result<bool> {
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

    /// `filename` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/storage#stat>
    pub fn file_stat(&mut self, mut filename: String) -> Result<FileStat> {
        filename.push('\0');

        let mut stat = sys::DiscordFileStat::default();

        unsafe {
            ffi!(self
                .get_storage_manager()
                .stat(filename.as_ptr() as *const _, &mut stat))
        }
        .to_result()?;

        Ok(stat.into())
    }

    /// <https://discordapp.com/developers/docs/game-sdk/storage#statat>  
    /// <https://discordapp.com/developers/docs/game-sdk/storage#count>
    pub fn all_file_stats(&mut self) -> Result<Vec<FileStat>> {
        let mut count = 0;

        unsafe { ffi!(self.get_storage_manager().count(&mut count)) }

        let mut result = Vec::with_capacity(count as usize);
        let mut stat = sys::DiscordFileStat::default();

        for index in 0..count {
            unsafe { ffi!(self.get_storage_manager().stat_at(index as i32, &mut stat)) }
                .to_result()?;

            result.push(stat.into())
        }

        Ok(result)
    }

    /// <https://discordapp.com/developers/docs/game-sdk/storage#getpath>
    pub fn folder_path(&mut self) -> Result<String> {
        let mut path: sys::DiscordPath = [0; size_of::<sys::DiscordPath>()];

        unsafe { ffi!(self.get_storage_manager().get_path(&mut path)) }.to_result()?;

        Ok(charbuf_to_str(&path[..charbuf_len(&path)]).to_string())
    }
}
