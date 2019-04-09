use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntitlementCreate {
    pub entitlement: Entitlement,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntitlementDelete {
    pub entitlement: Entitlement,
}
