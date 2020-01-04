use crate::{
    callbacks::ResultFromCallback, sys, to_result::ToResult, Discord, FetchKind, Image,
    ImageHandle, Result,
};
use std::convert::TryFrom;

/// # Images
///
/// <https://discordapp.com/developers/docs/game-sdk/images>
impl<'a> Discord<'a> {
    /// Prepares an image.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/images#fetch>
    pub fn fetch_image(
        &self,
        handle: ImageHandle,
        refresh: FetchKind,
        callback: impl 'a + FnMut(&Discord<'_>, Result<ImageHandle>),
    ) {
        unsafe {
            ffi!(self
                .get_image_manager()
                .fetch(handle.into(), refresh.into())
                .and_then(ResultFromCallback::new(callback)))
        }
    }

    /// Get's the dimensions of the source image.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/images#getdimensions>
    pub fn image_dimensions(&self, handle: ImageHandle) -> Result<(u32, u32)> {
        let mut dimensions = sys::DiscordImageDimensions::default();

        unsafe {
            ffi!(self
                .get_image_manager()
                .get_dimensions(handle.into(), &mut dimensions,))
        }
        .to_result()?;

        Ok((dimensions.width, dimensions.height))
    }

    /// Retrieves the data for an image.
    ///
    /// The image must be [fetched](#method.fetch_image) first.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/images#getdata>
    // TODO: example using image crate
    pub fn image(&self, handle: ImageHandle) -> Result<Image> {
        let (width, height) = self.image_dimensions(handle)?;
        let mut data: Vec<u8> = vec![0; 4 * width as usize * height as usize];

        debug_assert!(u32::try_from(data.len()).is_ok());

        unsafe {
            ffi!(self.get_image_manager().get_data(
                handle.into(),
                data.as_mut_ptr(),
                data.len() as u32
            ))
        }
        .to_result()?;

        Ok(Image {
            width,
            height,
            data,
        })
    }
}
