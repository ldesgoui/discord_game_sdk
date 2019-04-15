use crate::Relationship;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Refresh;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Update {
    pub relationship: Relationship,
}
