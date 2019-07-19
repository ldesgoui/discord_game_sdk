use crate::{
    callbacks::{ResultBytesCallback, ResultCallback},
    sys,
    to_result::ToResult,
    utils::cstr_to_str,
    Discord, FileStat, Result,
};
use std::ffi::CStr;
use std::mem::size_of;

/// # Storage
///
/// <https://discordapp.com/developers/docs/game-sdk/storage>
impl<'a> Discord<'a> {
    /// <https://discordapp.com/developers/docs/game-sdk/storage#read>
    pub fn read_file(
        &mut self,
        filename: impl AsRef<CStr>,
        mut buffer: impl AsMut<[u8]>,
    ) -> Result<u32> {
        let mut read = 0;

        let buffer = buffer.as_mut();

        unsafe {
            ffi!(self.get_storage_manager().read(
                filename.as_ref().as_ptr(),
                buffer.as_mut_ptr(),
                std::cmp::min(buffer.len(), u32::max_value() as usize) as u32,
                &mut read as *mut _
            ))
        }
        .to_result()?;

        Ok(read)
    }

    /// <https://discordapp.com/developers/docs/game-sdk/storage#readasync>
    pub fn read_file_async(
        &mut self,
        filename: impl AsRef<CStr>,
        callback: impl FnMut(&mut Discord, Result<Vec<u8>>) + 'a,
    ) {
        unsafe {
            ffi!(self
                .get_storage_manager()
                .read_async(filename.as_ref().as_ptr())
                .and_then(ResultBytesCallback::new(callback)))
        }
    }

    /// <https://discordapp.com/developers/docs/game-sdk/storage#readasyncpartial>
    pub fn read_file_async_partial(
        &mut self,
        filename: impl AsRef<CStr>,
        offset: u64,
        length: u64,
        callback: impl FnMut(&mut Discord, Result<Vec<u8>>) + 'a,
    ) {
        unsafe {
            ffi!(self
                .get_storage_manager()
                .read_async_partial(filename.as_ref().as_ptr(), offset, length)
                .and_then(ResultBytesCallback::new(callback)))
        }
    }

    /// <https://discordapp.com/developers/docs/game-sdk/storage#write>
    pub fn write_file(
        &mut self,
        filename: impl AsRef<CStr>,
        buffer: impl AsRef<[u8]>,
    ) -> Result<()> {
        let buffer = buffer.as_ref();
        unsafe {
            ffi!(self.get_storage_manager().write(
                filename.as_ref().as_ptr(),
                buffer.as_ptr() as *mut _,
                buffer.len() as u32,
            ))
        }
        .to_result()
    }

    /// <https://discordapp.com/developers/docs/game-sdk/storage#writeasync>
    pub fn write_file_async(
        &mut self,
        filename: impl AsRef<CStr>,
        buffer: impl AsRef<[u8]>,
        callback: impl FnMut(&mut Discord, Result<()>) + 'a,
    ) {
        let buffer = buffer.as_ref();
        unsafe {
            ffi!(self
                .get_storage_manager()
                .write_async(
                    filename.as_ref().as_ptr(),
                    buffer.as_ptr() as *mut _,
                    buffer.len() as u32
                )
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// <https://discordapp.com/developers/docs/game-sdk/storage#delete>
    pub fn delete_file(&mut self, filename: impl AsRef<CStr>) -> Result<()> {
        unsafe {
            ffi!(self
                .get_storage_manager()
                .delete_(filename.as_ref().as_ptr()))
        }
        .to_result()
    }

    /// <https://discordapp.com/developers/docs/game-sdk/storage#exists>
    pub fn file_exists(&mut self, filename: impl AsRef<CStr>) -> Result<bool> {
        let mut exists = false;

        unsafe {
            ffi!(self
                .get_storage_manager()
                .exists(filename.as_ref().as_ptr(), &mut exists))
        }
        .to_result()?;

        Ok(exists)
    }

    /// <https://discordapp.com/developers/docs/game-sdk/storage#stat>
    pub fn file_stat(&mut self, filename: impl AsRef<CStr>) -> Result<FileStat> {
        let mut stat = FileStat(sys::DiscordFileStat::default());

        unsafe {
            ffi!(self
                .get_storage_manager()
                .stat(filename.as_ref().as_ptr(), &mut stat.0))
        }
        .to_result()?;

        Ok(stat)
    }

    /// <https://discordapp.com/developers/docs/game-sdk/storage#statat>  
    /// <https://discordapp.com/developers/docs/game-sdk/storage#count>
    pub fn all_file_stats(&mut self) -> Result<Vec<FileStat>> {
        let mut count = 0;

        unsafe { ffi!(self.get_storage_manager().count(&mut count)) }

        let mut result = Vec::with_capacity(count as usize);
        let mut stat = FileStat(sys::DiscordFileStat::default());

        for index in 0..count {
            unsafe {
                ffi!(self
                    .get_storage_manager()
                    .stat_at(index as i32, &mut stat.0))
            }
            .to_result()?;

            result.push(stat)
        }

        Ok(result)
    }

    /// <https://discordapp.com/developers/docs/game-sdk/storage#getpath>
    pub fn folder_path(&mut self) -> Result<String> {
        let mut path: sys::DiscordPath = [0; size_of::<sys::DiscordPath>()];

        unsafe { ffi!(self.get_storage_manager().get_path(&mut path)) }.to_result()?;

        Ok(cstr_to_str(&path[..]).to_string())
    }
}
