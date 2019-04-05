use crate::prelude::*;

/// # Storage
impl<'a> Discord<'a> {
    pub fn read_file<S>(&mut self, filename: S, buffer: &mut [u8]) -> Result<u32>
    where
        S: AsRef<str>,
    {
        let filename = std::ffi::CString::new(filename.as_ref()).unwrap();
        let mut read = 0u32;

        unsafe {
            ffi!(self.get_storage_manager().read(
                filename.as_ptr(),
                buffer.as_mut_ptr(),
                std::cmp::min(buffer.len(), u32::max_value() as usize) as u32,
                &mut read as *mut _
            ))
        }
        .to_result()?;

        Ok(read)
    }

    pub fn read_file_async<S, F>(&mut self, filename: S, callback: F)
    where
        S: AsRef<str>,
        F: FnMut(Result<&[u8]>),
    {
        let filename = std::ffi::CString::new(filename.as_ref()).unwrap();

        unsafe {
            ffi!(self.get_storage_manager().read_async(
                filename.as_ptr(),
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::slice::<F>)
            ))
        }
    }

    pub fn read_file_async_partial<S, F>(
        &mut self,
        filename: S,
        offset: u64,
        length: u64,
        callback: F,
    ) where
        S: AsRef<str>,
        F: FnMut(Result<&[u8]>),
    {
        let filename = std::ffi::CString::new(filename.as_ref()).unwrap();

        unsafe {
            ffi!(self.get_storage_manager().read_async_partial(
                filename.as_ptr(),
                offset,
                length,
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::slice::<F>)
            ))
        }
    }

    pub fn write_file<S>(&mut self, filename: S, buffer: &[u8]) -> Result<()>
    where
        S: AsRef<str>,
    {
        let filename = std::ffi::CString::new(filename.as_ref()).unwrap();

        unsafe {
            ffi!(self.get_storage_manager().write(
                filename.as_ptr(),
                buffer.as_ptr() as *mut _,
                buffer.len() as u32,
            ))
        }
        .to_result()
    }

    pub fn write_file_async<S, F>(&mut self, filename: S, buffer: &[u8], callback: F)
    where
        S: AsRef<str>,
        F: FnMut(Result<()>),
    {
        let filename = std::ffi::CString::new(filename.as_ref()).unwrap();

        unsafe {
            ffi!(self.get_storage_manager().write_async(
                filename.as_ptr(),
                buffer.as_ptr() as *mut _,
                buffer.len() as u32,
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result::<F>)
            ))
        }
    }

    pub fn delete_file<S>(&mut self, filename: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let filename = std::ffi::CString::new(filename.as_ref()).unwrap();

        unsafe { ffi!(self.get_storage_manager().delete_(filename.as_ptr())) }.to_result()
    }

    pub fn file_exists<S>(&mut self, filename: S) -> Result<bool>
    where
        S: AsRef<str>,
    {
        let filename = std::ffi::CString::new(filename.as_ref()).unwrap();
        let mut exists = false;

        unsafe {
            ffi!(self
                .get_storage_manager()
                .exists(filename.as_ptr(), &mut exists))
        }
        .to_result()?;

        Ok(exists)
    }

    pub fn file_stat<S>(&mut self, filename: S) -> Result<FileStat>
    where
        S: AsRef<str>,
    {
        let filename = std::ffi::CString::new(filename.as_ref()).unwrap();
        let mut stat = sys::DiscordFileStat::default();

        unsafe {
            ffi!(self
                .get_storage_manager()
                .stat(filename.as_ptr(), &mut stat))
        }
        .to_result()?;

        Ok(FileStat::from_sys(&stat))
    }

    pub fn all_file_stats<F>(&mut self) -> Result<Vec<FileStat>> {
        let mut count = 0;

        unsafe { ffi!(self.get_storage_manager().count(&mut count)) }

        let mut result = Vec::with_capacity(count as usize);
        let mut stat = sys::DiscordFileStat::default();

        for index in 0..count {
            unsafe { ffi!(self.get_storage_manager().stat_at(index as i32, &mut stat)) }
                .to_result()?;

            result.push(FileStat::from_sys(&stat))
        }

        Ok(result)
    }

    pub fn folder_path(&mut self) -> Result<String> {
        let mut path: sys::DiscordPath = [0; 4096];

        unsafe { ffi!(self.get_storage_manager().get_path(&mut path)) }.to_result()?;

        Ok(unsafe { string_from_cstr(&path as *const _) })
    }
}