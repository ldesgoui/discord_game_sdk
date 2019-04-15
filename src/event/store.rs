use crate::Entitlement;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EntitlementCreate {
    pub entitlement: Entitlement,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EntitlementDelete {
    pub entitlement: Entitlement,
}
