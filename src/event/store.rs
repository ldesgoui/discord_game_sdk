use crate::Entitlement;

/// On Store Entitlement Create
///
/// <https://discordapp.com/developers/docs/game-sdk/store#onentitlementcreate>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EntitlementCreate {
    pub entitlement: Entitlement,
}

/// On Store Entitlement Delete
///
/// <https://discordapp.com/developers/docs/game-sdk/store#onentitlementdelete>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EntitlementDelete {
    pub entitlement: Entitlement,
}
