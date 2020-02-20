use crate::{callback, iter, sys, to_result::ToResult, utils, Discord, FileStat, Result};
use std::{
    borrow::Cow,
    convert::{TryFrom, TryInto},
    mem::size_of,
};

/// # Storage
///
/// > [Chapter in official docs](https://discordapp.com/developers/docs/game-sdk/storage)
impl Discord {
    /// Reads data synchronously from the game's allocated save file into a buffer.
    ///
    /// The file is mapped by key-value pair, and this function will read data that exists
    /// for the given key name.
    ///
    /// `buffer` should not exceed 4 294 967 295 bytes.
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `filename` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#read)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// let mut contents = vec![0_u8; 2048];
    ///
    /// discord.read_file("profile_1.save\0", &mut contents);
    /// # Ok(()) }
    pub fn read_file<'s>(
        &self,
        filename: impl Into<Cow<'s, str>>,
        mut buffer: impl AsMut<[u8]>,
    ) -> Result<u64> {
        let mut filename = filename.into();

        if !filename.ends_with('\0') {
            filename.to_mut().push('\0')
        };

        let mut read = 0;

        let buffer = buffer.as_mut();

        debug_assert!(u32::try_from(buffer.len()).is_ok());

        self.with_storage_manager(|mgr| unsafe {
            mgr.read.unwrap()(
                mgr,
                filename.as_ptr(),
                buffer.as_mut_ptr(),
                // XXX: u32 should be u64
                buffer.len().try_into().unwrap_or(u32::max_value()),
                &mut read,
            )
        })
        .to_result()?;

        // XXX: u32 should be u64
        Ok(read.try_into().unwrap())
    }

    /// Reads data asynchronously from the game's allocated save file into a buffer.
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `filename` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#readasync)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.read_file_async("profile_1.save\0", |discord, contents| {
    ///     match contents {
    ///         Ok(contents) => println!("read {} bytes", contents.len()),
    ///         Err(error) => eprintln!("failed to read file: {}", error),
    ///     }
    /// });
    /// # Ok(()) }
    pub fn read_file_async<'d, 's>(
        &'d self,
        filename: impl Into<Cow<'s, str>>,
        callback: impl 'd + FnOnce(&Self, Result<&[u8]>),
    ) {
        let mut filename = filename.into();

        if !filename.ends_with('\0') {
            filename.to_mut().push('\0')
        };

        self.with_storage_manager(|mgr| {
            let (ptr, fun) =
                callback::three_params(|res: sys::EDiscordResult, data: *mut u8, data_len: u32| {
                    callback(
                        self,
                        res.to_result().map(|()| unsafe {
                            std::slice::from_raw_parts(data, data_len as usize)
                        }),
                    )
                });

            unsafe { mgr.read_async.unwrap()(mgr, filename.as_ptr(), ptr, fun) }
        })
    }

    /// Reads data asynchronously from the game's allocated save file into a buffer,
    /// starting at a given offset and up to a given length.
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `filename` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#readasyncpartial)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.read_file_async_partial("profile_1.save\0", 30, 10, |discord, contents| {
    ///     match contents {
    ///         Ok(contents) => println!("read {} bytes", contents.len()),
    ///         Err(error) => eprintln!("failed to partially read file: {}", error),
    ///     }
    /// });
    /// # Ok(()) }
    pub fn read_file_async_partial<'d, 's>(
        &'d self,
        filename: impl Into<Cow<'s, str>>,
        offset: u64,
        length: u64,
        callback: impl 'd + FnOnce(&Self, Result<&[u8]>),
    ) {
        let mut filename = filename.into();

        if !filename.ends_with('\0') {
            filename.to_mut().push('\0')
        };

        self.with_storage_manager(|mgr| {
            let (ptr, fun) =
                callback::three_params(|res: sys::EDiscordResult, data: *mut u8, data_len: u32| {
                    callback(
                        self,
                        res.to_result().map(|()| unsafe {
                            std::slice::from_raw_parts(data, data_len as usize)
                        }),
                    )
                });

            unsafe {
                mgr.read_async_partial.unwrap()(mgr, filename.as_ptr(), offset, length, ptr, fun)
            }
        })
    }

    /// Writes data synchronously to disk, under the given key name.
    ///
    /// `buffer` should not exceed 4 294 967 295 bytes.
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `filename` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#write)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// let contents = "important save data".as_bytes();
    ///
    /// discord.write_file("profile_1.save\0", contents)?;
    /// # Ok(()) }
    pub fn write_file<'s>(
        &self,
        filename: impl Into<Cow<'s, str>>,
        buffer: impl AsRef<[u8]>,
    ) -> Result<()> {
        let mut filename = filename.into();

        if !filename.ends_with('\0') {
            filename.to_mut().push('\0')
        };

        let buffer = buffer.as_ref();

        debug_assert!(u32::try_from(buffer.len()).is_ok());

        self.with_storage_manager(|mgr| unsafe {
            mgr.write.unwrap()(
                mgr,
                filename.as_ptr(),
                // XXX: *mut should be *const
                buffer.as_ptr() as *mut u8,
                // XXX: u32 should be u64
                buffer.len().try_into().unwrap_or(u32::max_value()),
            )
        })
        .to_result()
    }

    /// Writes data asynchronously to disk under the given key.
    ///
    /// `buffer` should not exceed 4 294 967 295 bytes.
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `filename` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#writeasync)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// let contents = "important save data".as_bytes();
    ///
    /// discord.write_file_async("profile_1.save\0", contents, |discord, res| {
    ///     if let Err(error) = res {
    ///         eprintln!("failed to write to file: {}", error);
    ///     }
    /// });
    /// # Ok(()) }
    pub fn write_file_async<'d, 's>(
        &'d self,
        filename: impl Into<Cow<'s, str>>,
        buffer: impl AsRef<[u8]>,
        callback: impl 'd + FnOnce(&Self, Result<()>),
    ) {
        let mut filename = filename.into();

        if !filename.ends_with('\0') {
            filename.to_mut().push('\0')
        };

        let buffer = buffer.as_ref();

        debug_assert!(u32::try_from(buffer.len()).is_ok());

        self.with_storage_manager(|mgr| {
            let (ptr, fun) =
                callback::one_param(|res: sys::EDiscordResult| callback(self, res.to_result()));

            unsafe {
                mgr.write_async.unwrap()(
                    mgr,
                    filename.as_ptr(),
                    // XXX: *mut should be *const
                    buffer.as_ptr() as *mut u8,
                    // XXX: u32 should be u64
                    buffer.len().try_into().unwrap_or(u32::max_value()),
                    ptr,
                    fun,
                )
            }
        })
    }

    /// Deletes written data for the given key.
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `filename` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#delete)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.delete_file("profile_1.save\0")?;
    /// # Ok(()) }
    pub fn delete_file<'s>(&self, filename: impl Into<Cow<'s, str>>) -> Result<()> {
        let mut filename = filename.into();

        if !filename.ends_with('\0') {
            filename.to_mut().push('\0')
        };

        self.with_storage_manager(|mgr| unsafe { mgr.delete_.unwrap()(mgr, filename.as_ptr()) })
            .to_result()
    }

    /// Checks if data exists for a given key.
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `filename` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#exists)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// if discord.file_exists("profile_1.save\0")? {
    ///     // ...
    /// }
    /// # Ok(()) }
    pub fn file_exists<'s>(&self, filename: impl Into<Cow<'s, str>>) -> Result<bool> {
        let mut filename = filename.into();

        if !filename.ends_with('\0') {
            filename.to_mut().push('\0')
        };

        let mut exists = false;

        self.with_storage_manager(|mgr| unsafe {
            mgr.exists.unwrap()(mgr, filename.as_ptr(), &mut exists)
        })
        .to_result()?;

        Ok(exists)
    }

    /// Returns file info for the given key.
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `filename` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#stat)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// let file_stat = discord.file_stat("profile_1.save\0")?;
    /// # Ok(()) }
    pub fn file_stat<'s>(&self, filename: impl Into<Cow<'s, str>>) -> Result<FileStat> {
        let mut filename = filename.into();

        if !filename.ends_with('\0') {
            filename.to_mut().push('\0')
        };

        let mut stat = FileStat(sys::DiscordFileStat::default());

        self.with_storage_manager(|mgr| unsafe {
            mgr.stat.unwrap()(mgr, filename.as_ptr(), &mut stat.0)
        })
        .to_result()?;

        Ok(stat)
    }

    /// Returns the number of file stats.
    ///
    /// Prefer using [`iter_file_stats`](#method.iter_file_stats).
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#count)
    pub fn file_stat_count(&self) -> u32 {
        let mut count = 0;

        self.with_storage_manager(|mgr| unsafe { mgr.count.unwrap()(mgr, &mut count) });

        // XXX: i32 should be u32
        count.try_into().unwrap()
    }

    /// Returns the file stat at a given index.
    ///
    /// Prefer using [`iter_file_stats`](#method.iter_file_stats).
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#statat)  
    pub fn file_stat_at(&self, index: u32) -> Result<FileStat> {
        let mut stat = FileStat(sys::DiscordFileStat::default());

        self.with_storage_manager(|mgr| unsafe {
            mgr.stat_at.unwrap()(
                mgr,
                // XXX: i32 should be u32
                index.try_into().unwrap(),
                &mut stat.0,
            )
        })
        .to_result()?;

        Ok(stat)
    }

    /// Returns an `Iterator` over file stats.
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// for file_stat in discord.iter_file_stats() {
    ///     let file_stat = file_stat?;
    ///     // ...
    /// }
    /// # Ok(()) }
    pub fn iter_file_stats<'d>(
        &'d self,
    ) -> impl 'd
           + Iterator<Item = Result<FileStat>>
           + DoubleEndedIterator
           + ExactSizeIterator
           + std::iter::FusedIterator
           + std::fmt::Debug {
        iter::Collection::new(self, Self::file_stat_at, self.file_stat_count())
    }

    /// Returns the path to the folder where files are stored.
    /// It is specific to the application ID, the current branch, and the current user.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/storage#getpath)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// let folder_path = discord.folder_path()?;
    /// # Ok(()) }
    pub fn folder_path(&self) -> Result<String> {
        let mut path: sys::DiscordPath = [0; size_of::<sys::DiscordPath>()];

        self.with_storage_manager(|mgr| unsafe { mgr.get_path.unwrap()(mgr, &mut path) })
            .to_result()?;

        Ok(utils::charbuf_to_str(&path).to_string())
    }
}
