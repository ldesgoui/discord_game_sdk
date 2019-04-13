use crate::sys;

#[derive(Clone, Copy, Debug, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct Entitlement(pub(crate) sys::DiscordEntitlement);
