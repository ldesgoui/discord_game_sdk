use crate::prelude::*;

/// # Images
impl<'a> Discord<'a> {
    pub fn fetch_image<F>(&mut self, handle: ImageHandle, refresh: bool, callback: F)
    where
        F: FnMut(&mut Discord, Result<ImageHandle>),
    {
        unsafe {
            ffi!(self.get_image_manager().fetch(
                handle.to_sys(),
                refresh,
                self.wrap_callback(callback),
                Some(callbacks::result_from_sys::<F, ImageHandle>),
            ))
        }
    }

    pub fn image_dimensions(&mut self, handle: ImageHandle) -> Result<(u32, u32)> {
        let mut dimensions = sys::DiscordImageDimensions::default();

        unsafe {
            ffi!(self
                .get_image_manager()
                .get_dimensions(handle.to_sys(), &mut dimensions,))
        }
        .to_result()?;

        Ok((dimensions.width, dimensions.height))
    }

    pub fn image_data(&mut self, handle: ImageHandle) -> Result<Vec<u8>> {
        let (w, h) = self.image_dimensions(handle)?;
        let mut buf: Vec<u8> = vec![0; (w * h) as usize];

        unsafe {
            ffi!(self.get_image_manager().get_data(
                handle.to_sys(),
                buf[..].as_mut_ptr(),
                buf.len() as u32
            ))
        }
        .to_result()?;

        Ok(buf)
    }
}
