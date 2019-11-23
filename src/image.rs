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
    pub fn data(&'_ self) -> &'_ [u8] {
        &self.data
    }

    /// Get R, G, B, A channels of pixel at (x, y)
    pub fn pixel(&self, x: u32, y: u32) -> (u8, u8, u8, u8) {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);

        let idx = x as usize + y as usize * self.width as usize;

        (
            self.data[idx],
            self.data[idx + 1],
            self.data[idx + 2],
            self.data[idx + 3],
        )
    }
}
