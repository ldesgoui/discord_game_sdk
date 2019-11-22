/// Image Fetch Option
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FetchKind {
    ForceRefresh,
    UseCached,
}

#[doc(hidden)]
impl Into<bool> for FetchKind {
    fn into(self) -> bool {
        match self {
            Self::ForceRefresh => true,
            Self::UseCached => false,
        }
    }
}
