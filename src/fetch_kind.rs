#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FetchKind {
    ForceRefresh,
    UseCached,
}

#[doc(hidden)]
impl Into<bool> for FetchKind {
    fn into(self) -> bool {
        match self {
            FetchKind::ForceRefresh => true,
            FetchKind::UseCached => false,
        }
    }
}
