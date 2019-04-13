use crate::{
    callbacks::{ResultBytesCallback, ResultCallback},
    sys,
    to_result::ToResult,
    Discord, DiscordResult, FileStat,
};
use std::ffi::CStr;
use std::mem::{size_of, transmute};

/// # Storage
impl<'a> Discord<'a> {
    // tested
    pub fn read_file(
        &mut self,
        filename: impl AsRef<CStr>,
        buffer: &mut [u8],
    ) -> DiscordResult<u32> {
        let mut read = 0;

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
                .read_async(filename.as_ref().as_ptr())(
                ResultBytesCallback::new(callback)
            ))
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
            ffi!(self.get_storage_manager().read_async_partial(
                filename.as_ref().as_ptr(),
                offset,
                length
            )(ResultBytesCallback::new(callback)))
        }
    }

    // tested
    pub fn write_file(&mut self, filename: impl AsRef<CStr>, buffer: &[u8]) -> DiscordResult<()> {
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
    pub fn write_file_async<F>(&mut self, filename: impl AsRef<CStr>, buffer: &[u8], callback: F)
    where
        F: FnMut(&mut Discord, DiscordResult<()>) + 'a,
    {
        unsafe {
            ffi!(self.get_storage_manager().write_async(
                filename.as_ref().as_ptr(),
                buffer.as_ptr() as *mut _,
                buffer.len() as u32
            )(ResultCallback::new(callback)))
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
        let mut stat = sys::DiscordFileStat::default();

        unsafe {
            ffi!(self
                .get_storage_manager()
                .stat(filename.as_ref().as_ptr(), &mut stat))
        }
        .to_result()?;

        Ok(FileStat::from(stat))
    }

    // tested
    pub fn all_file_stats(&mut self) -> DiscordResult<Vec<FileStat>> {
        let mut count = 0;

        unsafe { ffi!(self.get_storage_manager().count(&mut count)) }

        let mut result = Vec::with_capacity(count as usize);
        let mut stat = sys::DiscordFileStat::default();

        for index in 0..count {
            unsafe { ffi!(self.get_storage_manager().stat_at(index as i32, &mut stat)) }
                .to_result()?;

            result.push(FileStat::from(stat))
        }

        Ok(result)
    }

    // tested
    pub fn folder_path(&mut self) -> DiscordResult<String> {
        let mut path: sys::DiscordPath = [0; size_of::<sys::DiscordPath>()];

        unsafe { ffi!(self.get_storage_manager().get_path(&mut path)) }.to_result()?;

        Ok(CStr::from_bytes_with_nul(unsafe { transmute(&path[..]) })
            .unwrap()
            .to_str()
            .unwrap()
            .to_string())
    }
}
