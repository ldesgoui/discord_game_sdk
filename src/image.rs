#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Image {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) data: Vec<u8>,
}

impl Image {
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Flat slice of uncompressed SRGBA image data
    ///
    /// Length is `width * height * 4`
    ///
    /// Pattern is: `RGBARGBARGBA...`
    pub fn data(&'_ self) -> &'_ [u8] {
        &self.data[..]
    }
}
