use crate::{
    callbacks::{ResultBytesCallback, ResultCallback},
    sys,
    to_result::ToResult,
    utils::cstr_to_str,
    Discord, DiscordResult, FileStat,
};
use std::ffi::CStr;
use std::mem::size_of;

/// # Storage
impl<'a> Discord<'a> {
    // tested
    pub fn read_file(
        &mut self,
        filename: impl AsRef<CStr>,
        mut buffer: impl AsMut<[u8]>,
    ) -> DiscordResult<u32> {
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

    // tested
    pub fn read_file_async<F>(&mut self, filename: impl AsRef<CStr>, callback: F)
    where
        F: FnMut(&mut Discord, DiscordResult<Vec<u8>>) + 'a,
    {
        unsafe {
            ffi!(self
                .get_storage_manager()
                .read_async(filename.as_ref().as_ptr())
                .and_then(ResultBytesCallback::new(callback)))
        }
    }

    // tested
    pub fn read_file_async_partial<F>(
        &mut self,
        filename: impl AsRef<CStr>,
        offset: u64,
        length: u64,
        callback: F,
    ) where
        F: FnMut(&mut Discord, DiscordResult<Vec<u8>>) + 'a,
    {
        unsafe {
            ffi!(self
                .get_storage_manager()
                .read_async_partial(filename.as_ref().as_ptr(), offset, length)
                .and_then(ResultBytesCallback::new(callback)))
        }
    }

    // tested
    pub fn write_file(
        &mut self,
        filename: impl AsRef<CStr>,
        buffer: impl AsRef<[u8]>,
    ) -> DiscordResult<()> {
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

    // tested
    pub fn write_file_async<F>(
        &mut self,
        filename: impl AsRef<CStr>,
        buffer: impl AsRef<[u8]>,
        callback: F,
    ) where
        F: FnMut(&mut Discord, DiscordResult<()>) + 'a,
    {
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

    // tested
    pub fn delete_file(&mut self, filename: impl AsRef<CStr>) -> DiscordResult<()> {
        unsafe {
            ffi!(self
                .get_storage_manager()
                .delete_(filename.as_ref().as_ptr()))
        }
        .to_result()
    }

    // tested
    pub fn file_exists(&mut self, filename: impl AsRef<CStr>) -> DiscordResult<bool> {
        let mut exists = false;

        unsafe {
            ffi!(self
                .get_storage_manager()
                .exists(filename.as_ref().as_ptr(), &mut exists))
        }
        .to_result()?;

        Ok(exists)
    }

    // tested
    pub fn file_stat(&mut self, filename: impl AsRef<CStr>) -> DiscordResult<FileStat> {
        let mut stat = FileStat(sys::DiscordFileStat::default());

        unsafe {
            ffi!(self
                .get_storage_manager()
                .stat(filename.as_ref().as_ptr(), &mut stat.0))
        }
        .to_result()?;

        Ok(stat)
    }

    // tested
    pub fn all_file_stats(&mut self) -> DiscordResult<Vec<FileStat>> {
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

    // tested
    pub fn folder_path(&mut self) -> DiscordResult<String> {
        let mut path: sys::DiscordPath = [0; size_of::<sys::DiscordPath>()];

        unsafe { ffi!(self.get_storage_manager().get_path(&mut path)) }.to_result()?;

        Ok(cstr_to_str(&path[..]).to_string())
    }
}
