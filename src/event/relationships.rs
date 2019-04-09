use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Refresh;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Update {
    pub relationship: Relationship,
}
