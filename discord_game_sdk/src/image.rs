/// Image with pixel data
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Image {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) data: Vec<u8>,
}

impl Image {
    /// The width and height in pixels of the image
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// The width in pixels of the image
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height in pixels of the image
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Flat slice of uncompressed SRGBA image data
    ///
    /// Length is `width * height * 4`
    ///
    /// Pattern is: `RGBARGBARGBA...`
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

#[cfg(feature = "image")]
impl Into<image::RgbaImage> for Image {
    fn into(self) -> image::RgbaImage {
        image::RgbaImage::from_raw(self.width, self.height, self.data)
            .expect("discord_game_sdk: invalid size for image buffer")
    }
}
