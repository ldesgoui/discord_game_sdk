use crate::prelude::*;

/// # Storage
impl Discord {
    pub fn read_file<S>(&self, filename: S, buffer: &mut [u8]) -> Result<u32>
    where
        S: AsRef<str>,
    {
        let filename =
            std::ffi::CString::new(filename.as_ref()).map_err(DeveloperViolation::from)?;
        let mut read = 0u32;

        ffi!(self.get_storage_manager().read(
            filename.as_ptr(),
            buffer.as_mut_ptr(),
            std::cmp::min(buffer.len(), std::u32::MAX as usize) as u32,
            &mut read as *mut _
        ))?;

        Ok(read)
    }

    pub fn read_file_async<S, F>(&self, filename: S, mut callback: F)
    where
        S: AsRef<str>,
        F: FnMut(Result<&[u8]>),
    {
        let _ = || -> Result<()> {
            let filename =
                std::ffi::CString::new(filename.as_ref()).map_err(DeveloperViolation::from)?;

            ffi!(self.get_storage_manager().read_async(
                filename.as_ptr(),
                &mut callback as *mut _ as *mut _,
                Some(slice_callback::<F>)
            ))
        }()
        .map_err(|e| callback(Err(e)));
    }

    pub fn read_file_async_partial<S, F>(
        &self,
        filename: S,
        offset: u64,
        length: u64,
        mut callback: F,
    ) where
        S: AsRef<str>,
        F: FnMut(Result<&[u8]>),
    {
        let _ = || -> Result<()> {
            let filename =
                std::ffi::CString::new(filename.as_ref()).map_err(DeveloperViolation::from)?;

            ffi!(self.get_storage_manager().read_async_partial(
                filename.as_ptr(),
                offset,
                length,
                &mut callback as *mut _ as *mut _,
                Some(slice_callback::<F>)
            ))
        }()
        .map_err(|e| callback(Err(e)));
    }

    pub fn write_file<S>(&self, filename: S, buffer: &[u8]) -> Result<()>
    where
        S: AsRef<str>,
    {
        let filename =
            std::ffi::CString::new(filename.as_ref()).map_err(DeveloperViolation::from)?;

        ffi!(self.get_storage_manager().write(
            filename.as_ptr(),
            buffer.as_ptr() as *mut _,
            buffer.len() as u32,
        ))
    }

    pub fn write_file_async<S, F>(&self, filename: S, buffer: &[u8], mut callback: F)
    where
        S: AsRef<str>,
        F: FnMut(Result<()>),
    {
        let _ = || -> Result<()> {
            let filename =
                std::ffi::CString::new(filename.as_ref()).map_err(DeveloperViolation::from)?;

            ffi!(self.get_storage_manager().write_async(
                filename.as_ptr(),
                buffer.as_ptr() as *mut _,
                buffer.len() as u32,
                &mut callback as *mut _ as *mut _,
                Some(simple_callback::<F>)
            ))
        }()
        .map_err(|e| callback(Err(e)));
    }

    pub fn delete_file<S>(&self, filename: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let filename =
            std::ffi::CString::new(filename.as_ref()).map_err(DeveloperViolation::from)?;

        ffi!(self.get_storage_manager().delete_(filename.as_ptr()))
    }

    pub fn file_exists<S>(&self, filename: S) -> Result<bool>
    where
        S: AsRef<str>,
    {
        let filename =
            std::ffi::CString::new(filename.as_ref()).map_err(DeveloperViolation::from)?;
        let mut exists = false;

        ffi!(self
            .get_storage_manager()
            .exists(filename.as_ptr(), &mut exists))?;

        Ok(exists)
    }

    pub fn file_stat<S>(&self, filename: S) -> Result<FileStat>
    where
        S: AsRef<str>,
    {
        let filename =
            std::ffi::CString::new(filename.as_ref()).map_err(DeveloperViolation::from)?;
        let mut stat = sys::DiscordFileStat::default();

        ffi!(self
            .get_storage_manager()
            .stat(filename.as_ptr(), &mut stat))?;

        FileStat::from_sys(stat)
    }

    pub fn folder_path(&self) -> Result<String> {
        let mut path: sys::DiscordPath = [0; 4096];

        ffi!(self.get_storage_manager().get_path(&mut path))?;

        Ok(from_cstr(&path as *const _)?.to_string())
    }
}

extern "C" fn slice_callback<F>(
    data: *mut c_void,
    res: sys::EDiscordResult,
    buffer: *mut u8,
    len: u32,
) where
    F: FnMut(Result<&[u8]>) + Sized,
{
    if data.is_null() {
        log::error!("SDK invoked callback with null");
        return;
    }
    let callback: &mut F = unsafe { &mut *(data as *mut _) };

    callback(
        res.to_result()
            .and_then(|_| Ok(unsafe { std::slice::from_raw_parts(buffer, len as usize) })),
    );
}
