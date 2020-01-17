use crate::{sys, to_result::ToResult, Discord, FetchKind, Image, ImageHandle, Result};
use std::convert::TryFrom;

/// # Images
///
/// > [Chapter in official docs](https://discordapp.com/developers/docs/game-sdk/images)
///
/// ```rust
/// # use discord_game_sdk::*;
/// # fn example(discord: Discord) -> Result<()> {
/// # let user = User::from(discord_game_sdk_sys::DiscordUser::default());
/// discord.fetch_image(
///     ImageHandle::from_user_id(user.id(), 128),
///     FetchKind::UseCached,
///     |discord, handle| {
///         match handle {
///             Ok(handle) => {
///                 println!("image dimensions: {:?}", discord.image_dimensions(handle));
///                 let image = discord.image(handle);
///                 // ...
///             },
///             Err(error) => eprintln!("failed to fetch image: {}", error),
///         }
///     },
/// );
/// # Ok(()) }
impl Discord {
    /// Prepares an image.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/images#fetch)
    pub fn fetch_image<'d>(
        &'d self,
        handle: ImageHandle,
        refresh: FetchKind,
        callback: impl 'd + FnOnce(&Self, Result<ImageHandle>),
    ) {
        unsafe {
            ffi!(self
                .get_image_manager()
                .fetch(handle.into(), refresh.into())
                .and_then(
                    |res: sys::EDiscordResult, image_handle: sys::DiscordImageHandle| {
                        callback::<Result<ImageHandle>>(
                            res.to_result().map(|()| image_handle.into()),
                        )
                    }
                ))
        }
    }

    /// Get's the dimensions of the source image.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/images#getdimensions)
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
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/images#getdata)
    pub fn image(&self, handle: ImageHandle) -> Result<Image> {
        let (width, height) = self.image_dimensions(handle)?;
        let mut data = vec![0u8; 4 * width as usize * height as usize];

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
