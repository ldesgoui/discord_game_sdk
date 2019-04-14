use crate::Entitlement;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EntitlementCreate {
    pub entitlement: Entitlement,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EntitlementDelete {
    pub entitlement: Entitlement,
}
