use crate::{
    sys, to_result::ToResult, utils::charbuf_to_str, Collection, Discord, FileStat, Result,
};
use std::{borrow::Cow, convert::TryFrom, mem::size_of};

/// # Storage
///
/// > [Chapter in official docs](https://discordapp.com/developers/docs/game-sdk/storage)
impl Discord {
    /// Reads data synchronously from the game's allocated save file into a buffer.
    /// The file is mapped by key-value pair, and this function will read data that exists
    /// for the given key name.
    ///
    /// `buffer` should not exceed 4 294 967 295 bytes.
    ///
    /// A nul byte will be appended to `filename` if necessary.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#read)
    pub fn read_file<'b>(
        &self,
        filename: impl Into<Cow<'b, str>>,
        mut buffer: impl AsMut<[u8]>,
    ) -> Result<usize> {
        let mut filename = filename.into();

        if !filename.ends_with('\0') {
            filename.to_mut().push('\0')
        };

        let mut read = 0;

        let buffer = buffer.as_mut();

        debug_assert!(u32::try_from(buffer.len()).is_ok());

        unsafe {
            ffi!(self.get_storage_manager().read(
                filename.as_ptr(),
                buffer.as_mut_ptr(),
                // u32 should be usize
                buffer.len() as u32,
                &mut read
            ))
        }
        .to_result()?;

        // XXX: u32 should be usize
        Ok(read as usize)
    }

    /// Reads data asynchronously from the game's allocated save file into a buffer.
    ///
    /// A nul byte will be appended to `filename` if necessary.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#readasync)
    pub fn read_file_async<'b>(
        &self,
        filename: impl Into<Cow<'b, str>>,
        callback: impl 'static + FnOnce(&Self, Result<&[u8]>),
    ) {
        let mut filename = filename.into();

        if !filename.ends_with('\0') {
            filename.to_mut().push('\0')
        };

        unsafe {
            ffi!(self
                .get_storage_manager()
                .read_async(filename.as_ptr())
                .and_then(|res: sys::EDiscordResult, data: *mut u8, data_len: u32| {
                    callback::<Result<&[u8]>>(
                        res.to_result()
                            .map(|()| std::slice::from_raw_parts(data, data_len as usize)),
                    )
                }))
        }
    }

    /// Reads data asynchronously from the game's allocated save file into a buffer,
    /// starting at a given offset and up to a given length.
    ///
    /// A nul byte will be appended to `filename` if necessary.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#readasyncpartial)
    pub fn read_file_async_partial<'b>(
        &self,
        filename: impl Into<Cow<'b, str>>,
        offset: usize,
        length: usize,
        callback: impl 'static + FnOnce(&Self, Result<&[u8]>),
    ) {
        let mut filename = filename.into();

        if !filename.ends_with('\0') {
            filename.to_mut().push('\0')
        };

        unsafe {
            ffi!(self
                .get_storage_manager()
                .read_async_partial(
                    filename.as_ptr(),
                    // XXX: u64 should be usize
                    offset as u64,
                    // XXX: u64 should be usize
                    length as u64
                )
                .and_then(|res: sys::EDiscordResult, data: *mut u8, data_len: u32| {
                    callback::<Result<&[u8]>>(
                        res.to_result()
                            .map(|()| std::slice::from_raw_parts(data, data_len as usize)),
                    )
                }))
        }
    }

    /// Writes data synchronously to disk, under the given key name.
    ///
    /// `buffer` should not exceed 4 294 967 295 bytes.
    ///
    /// A nul byte will be appended to `filename` if necessary.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#write)
    pub fn write_file<'b>(
        &self,
        filename: impl Into<Cow<'b, str>>,
        buffer: impl AsRef<[u8]>,
    ) -> Result<()> {
        let mut filename = filename.into();

        if !filename.ends_with('\0') {
            filename.to_mut().push('\0')
        };

        let buffer = buffer.as_ref();

        debug_assert!(u32::try_from(buffer.len()).is_ok());

        unsafe {
            ffi!(self.get_storage_manager().write(
                filename.as_ptr(),
                // XXX: *mut should be *const
                buffer.as_ptr() as *mut _,
                // XXX: u32 should be usize
                buffer.len() as u32,
            ))
        }
        .to_result()
    }

    /// Writes data asynchronously to disk under the given key.
    ///
    /// `buffer` should not exceed 4 294 967 295 bytes.
    ///
    /// A nul byte will be appended to `filename` if necessary.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#writeasync)
    pub fn write_file_async<'b>(
        &self,
        filename: impl Into<Cow<'b, str>>,
        buffer: impl AsRef<[u8]>,
        callback: impl 'static + FnOnce(&Self, Result<()>),
    ) {
        let mut filename = filename.into();

        if !filename.ends_with('\0') {
            filename.to_mut().push('\0')
        };

        let buffer = buffer.as_ref();

        debug_assert!(u32::try_from(buffer.len()).is_ok());

        unsafe {
            ffi!(self
                .get_storage_manager()
                .write_async(
                    filename.as_ptr(),
                    // XXX: *mut should be *const
                    buffer.as_ptr() as *mut _,
                    // XXX: u32 should be usize
                    buffer.len() as u32
                )
                .and_then(|res: sys::EDiscordResult| callback::<Result<()>>(res.to_result())))
        }
    }

    /// Deletes written data for the given key.
    ///
    /// A nul byte will be appended to `filename` if necessary.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#delete)
    pub fn delete_file<'b>(&self, filename: impl Into<Cow<'b, str>>) -> Result<()> {
        let mut filename = filename.into();

        if !filename.ends_with('\0') {
            filename.to_mut().push('\0')
        };

        unsafe { ffi!(self.get_storage_manager().delete_(filename.as_ptr())) }.to_result()
    }

    /// Checks if data exists for a given key.
    ///
    /// A nul byte will be appended to `filename` if necessary.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#exists)
    pub fn file_exists<'b>(&self, filename: impl Into<Cow<'b, str>>) -> Result<bool> {
        let mut filename = filename.into();

        if !filename.ends_with('\0') {
            filename.to_mut().push('\0')
        };

        let mut exists = false;

        unsafe {
            ffi!(self
                .get_storage_manager()
                .exists(filename.as_ptr(), &mut exists))
        }
        .to_result()?;

        Ok(exists)
    }

    /// Returns file info for the given key.
    ///
    /// A nul byte will be appended to `filename` if necessary.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#stat)
    pub fn file_stat<'b>(&self, filename: impl Into<Cow<'b, str>>) -> Result<FileStat> {
        let mut filename = filename.into();

        if !filename.ends_with('\0') {
            filename.to_mut().push('\0')
        };

        let mut stat = FileStat(sys::DiscordFileStat::default());

        unsafe {
            ffi!(self
                .get_storage_manager()
                .stat(filename.as_ptr(), &mut stat.0))
        }
        .to_result()?;

        Ok(stat)
    }

    /// Returns the number of file stats.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#count)
    pub fn file_stat_count(&self) -> usize {
        let mut count = 0;

        unsafe { ffi!(self.get_storage_manager().count(&mut count)) }

        // XXX: i32 should be usize
        count as usize
    }

    /// Returns the file stat at a given index.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#statat)  
    pub fn file_stat_at(&self, index: usize) -> Result<FileStat> {
        let mut stat = FileStat(sys::DiscordFileStat::default());

        unsafe {
            ffi!(self.get_storage_manager().stat_at(
                // XXX: i32 should be usize
                index as i32,
                &mut stat.0
            ))
        }
        .to_result()?;

        Ok(stat)
    }

    /// Returns an `Iterator` over file stats.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#count)
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#statat)  
    pub fn iter_file_stats(&self) -> Collection<Result<FileStat>> {
        Collection::new(self, Box::new(Self::file_stat_at), self.file_stat_count())
    }

    /// Returns the path to the folder where files are stored.
    /// It is specific to the application ID, the current branch, and the current user.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#getpath)
    pub fn folder_path(&self) -> Result<String> {
        let mut path: sys::DiscordPath = [0; size_of::<sys::DiscordPath>()];

        unsafe { ffi!(self.get_storage_manager().get_path(&mut path)) }.to_result()?;

        Ok(charbuf_to_str(&path).to_string())
    }
}
