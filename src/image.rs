#[derive(Clone, Debug, PartialEq, Eq)]
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

    pub fn data(&'_ self) -> &'_ [u8] {
        &self.data[..]
    }
}
