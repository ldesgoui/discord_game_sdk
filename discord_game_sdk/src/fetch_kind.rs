/// Image Fetch Option
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FetchKind {
    /// Always download a fresh version of the image
    ForceRefresh,
    /// Use a cached version of the image if available
    UseCached,
}

impl Into<bool> for FetchKind {
    fn into(self) -> bool {
        match self {
            Self::ForceRefresh => true,
            Self::UseCached => false,
        }
    }
}
